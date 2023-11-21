use crate::common::zrf;
use ndarray_npy::{NpzWriter, WriteNpzError};
use rayon::prelude::*;
use std::{fs::File, path::Path};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Unrecognized file extension: {0}")]
    FileExtensionError(String),
    #[error("Failed to parse .zrf file: {0}")]
    FileZRFParseError(#[from] zrf::Error),
    #[error("Failed to create file: {0}")]
    FileCreateError(#[from] std::io::Error),
    #[error("Failed to write to .npz: {0}")]
    FileWriteError(#[from] WriteNpzError),
}

#[tauri::command]
pub async fn convert(files: Vec<&str>, save_location: &str) -> Result<(), Vec<String>> {
    // Convert each file
    let results: Vec<String> = files
        .into_par_iter()
        .map(
            |file: &str| match Path::new(file).extension().unwrap().to_str().unwrap() {
                "zrf" => convert_zrf(file, save_location),
                ext => Err(Error::FileExtensionError(ext.to_string())),
            },
        )
        .filter_map(|out: Result<(), Error>| match out {
            Ok(_) => None,
            Err(e) => Some(e.to_string()),
        })
        .collect();

    if results.is_empty() {
        Ok(())
    } else {
        Err(results)
    }
}

fn convert_zrf(file: &str, save_location: &str) -> Result<(), Error> {
    // Parse .zrf file and get contents
    let out: zrf::Output = zrf::Parser::parse(file)?;

    // Create file
    let file_name = Path::new(file).file_name().unwrap();
    let save_location = Path::new(save_location)
        .join(file_name)
        .with_extension("npz");
    let mut npz = NpzWriter::new(File::create(save_location)?);

    // Save parsed contents into file
    npz.add_array("zsig", &out.zsig)?;
    npz.add_array("rfsig", &out.rfsig)?;
    npz.finish()?;

    // Success
    Ok(())
}
