use byteorder::{ByteOrder, LittleEndian};
use inner::inner;
use ndarray::{s, Array3};
use ndarray_stats::QuantileExt;
use std::fs;
use thiserror::Error;

const SIZE_16: usize = 2;
const SIZE_32: usize = 4;
const NUM_HEADER_ROWS: usize = 16;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Failed to open file: {0}")]
    FileError(#[from] std::io::Error),
    #[error("Failed to read: {0}")]
    ReadError(String),
    #[error("Failed to shape signals: {0}")]
    ShapeError(#[from] ndarray::ShapeError),
    #[error("Failed to find frame shift: {0}")]
    FrameShiftError(#[from] ndarray_stats::errors::MinMaxError),
}

enum DType {
    I16Vec(Vec<i16>),
    U16(u16),
    U16Vec(Vec<u16>),
    U32(u32),
}

#[derive(Debug)]
pub struct Output {
    // File info
    pub file_type: u32,
    pub file_version: u32,

    // SPSET info
    pub spset_version: u32,

    // Data dimensions
    pub num_scanlines: u16,
    pub num_frames: u32,
    pub num_depths_zsig: u32,
    pub num_depths_rfsig: u32,

    // Data
    pub zsig: Array3<u16>,
    pub rfsig: Array3<i16>,
}

pub struct Parser {}

impl Parser {
    pub fn parse(target: String) -> Result<Output, Error> {
        // Read raw bytes and initialize offset
        let bytes: Vec<u8> = fs::read(target)?;
        let mut offset: usize = 0;

        // Read file and SPSET information
        let file_type: u32 =
            inner!(Parser::read_int(&bytes, &mut offset, 0, "u32")?, if DType::U32);
        let file_version: u32 =
            inner!(Parser::read_int(&bytes, &mut offset, 0, "u32")?, if DType::U32);
        let _file_bytesize: u32 =
            inner!(Parser::read_int(&bytes, &mut offset, 8, "u32")?, if DType::U32);
        let spset_bytesize: u32 =
            inner!(Parser::read_int(&bytes, &mut offset, 0, "u32")?, if DType::U32);
        let spset_version: u32 =
            inner!(Parser::read_int(&bytes, &mut offset, 0, "u32")?, if DType::U32);

        // Read data dimensions
        let num_depths_zsig: u32 =
            inner!(Parser::read_int(&bytes, &mut offset, 56, "u32")?, if DType::U32);
        let num_depths_rfsig: u32 =
            inner!(Parser::read_int(&bytes, &mut offset, 0, "u32")?, if DType::U32);
        let num_scanlines_x_frames: u32 =
            inner!(Parser::read_int(&bytes, &mut offset, 0, "u32")?, if DType::U32);

        // Skip over SPSET and read ID size
        let id_bytesize: u32 = inner!(Parser::read_int(
            &bytes,
            &mut offset,
            isize::try_from(spset_bytesize).unwrap() - 76,
            "u32",
        )?, if DType::U32);

        // Skip over ID, read number of scanlines, and infer number of frames and elements
        let num_scanlines: u16 = inner!(Parser::read_int(
            &bytes,
            &mut offset,
            isize::try_from(id_bytesize).unwrap() + 6,
            "u16",
        )?, if DType::U16);
        let num_frames: u32 = num_scanlines_x_frames / u32::from(num_scanlines);
        let num_elements_zsig: u32 = num_depths_zsig * num_scanlines_x_frames;
        let num_elements_rfsig: u32 = num_depths_rfsig * num_scanlines_x_frames;

        // Read signals
        let zsig: Vec<u16> = inner!(Parser::read_vec(&bytes, &mut offset, -12, "u16", usize::try_from(num_elements_zsig).unwrap())?, if DType::U16Vec);
        let rfsig: Vec<i16> = inner!(Parser::read_vec(&bytes, &mut offset, 0, "i16", usize::try_from(num_elements_rfsig).unwrap())?, if DType::I16Vec);

        // Shape signals
        let mut zsig: Array3<u16> = Parser::shape_vec(
            zsig,
            usize::try_from(num_depths_zsig).unwrap(),
            usize::try_from(num_scanlines).unwrap(),
            usize::try_from(num_frames).unwrap(),
        )?;
        let mut rfsig: Array3<i16> = Parser::shape_vec(
            rfsig,
            usize::try_from(num_depths_rfsig).unwrap(),
            usize::try_from(num_scanlines).unwrap(),
            usize::try_from(num_frames).unwrap(),
        )?;

        // Get frame shift (from zsig) and correct frame order
        let frame_shift: isize = isize::try_from(Parser::find_frame_shift(&zsig)?).unwrap();
        zsig = Parser::unwrap_matrix(zsig, frame_shift);
        rfsig = Parser::unwrap_matrix(rfsig, frame_shift);

        // // Remove header
        zsig.slice_collapse(s![NUM_HEADER_ROWS.., .., ..]);
        rfsig.slice_collapse(s![NUM_HEADER_ROWS.., .., ..]);

        // Output
        Ok(Output {
            file_type,
            file_version,
            spset_version,
            num_scanlines,
            num_frames,
            num_depths_zsig: num_depths_zsig - u32::try_from(NUM_HEADER_ROWS).unwrap(),
            num_depths_rfsig: num_depths_rfsig - u32::try_from(NUM_HEADER_ROWS).unwrap(),
            zsig,
            rfsig,
        })
    }

    fn find_frame_shift<T: std::cmp::PartialOrd>(matrix: &Array3<T>) -> Result<usize, Error> {
        // Find beginning of circular buffer from certain row
        Ok(matrix.slice(s!(1, 0, ..)).argmin()?)
    }

    fn read_int(
        bytes: &Vec<u8>,
        offset: &mut usize,
        advance: isize,
        dtype: &str,
    ) -> Result<DType, Error> {
        // Advance offset (if necessary) and store value as index
        *offset = offset.checked_add_signed(advance).unwrap();
        let start: usize = *offset;

        // Read int
        match dtype {
            "u16" => {
                *offset += SIZE_16;
                Ok(DType::U16(LittleEndian::read_u16(&bytes[start..])))
            }
            "u32" => {
                *offset += SIZE_32;
                Ok(DType::U32(LittleEndian::read_u32(&bytes[start..])))
            }
            _ => Err(Error::ReadError(format!("Unknown {} dtype", dtype))),
        }
    }

    fn read_vec(
        bytes: &Vec<u8>,
        offset: &mut usize,
        advance: isize,
        dtype: &str,
        n: usize,
    ) -> Result<DType, Error> {
        // Advance offset (if necessary) and store value as index
        // Then increment offset to include number of specified elements (always i16 or u16)
        *offset = offset.checked_add_signed(advance).unwrap();
        let start: usize = *offset;
        *offset += 2 * n;

        // Read array
        match dtype {
            "i16" => {
                let mut out: Vec<i16> = vec![0; n];
                LittleEndian::read_i16_into(&bytes[start..*offset], &mut out);
                Ok(DType::I16Vec(out))
            }
            "u16" => {
                let mut out: Vec<u16> = vec![0; n];
                LittleEndian::read_u16_into(&bytes[start..*offset], &mut out);
                Ok(DType::U16Vec(out))
            }
            _ => Err(Error::ReadError(format!("Unknown {} dtype", dtype))),
        }
    }

    fn shape_vec<T>(
        vector: Vec<T>,
        num_depths: usize,
        num_scanlines: usize,
        num_frames: usize,
    ) -> Result<Array3<T>, Error> {
        // Read it into ndarray, which can handle dimensions
        let mut out: Array3<T> =
            Array3::from_shape_vec((num_frames, num_scanlines, num_depths), vector)?;
        // Make output shape (depths x scanline x frames)
        out.swap_axes(0, 2);
        Ok(out)
    }

    fn unwrap_matrix<T: std::clone::Clone>(matrix: Array3<T>, frame_shift: isize) -> Array3<T> {
        // Initialize a new array
        let mut out: Array3<std::mem::MaybeUninit<T>> = Array3::uninit(matrix.dim());

        // Frame shift (and onwards) are moved to front
        // Anything before frame shift is appended at the end
        matrix
            .slice(s![.., .., -frame_shift..])
            .assign_to(out.slice_mut(s![.., .., ..frame_shift]));
        matrix
            .slice(s![.., .., ..-frame_shift])
            .assign_to(out.slice_mut(s![.., .., frame_shift..]));

        // Promise that result is safe to use
        unsafe { out.assume_init() }
    }
}
