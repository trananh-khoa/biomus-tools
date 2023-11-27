<!-- Improved compatibility of back to top link: See: https://github.com/othneildrew/Best-README-Template/pull/73 -->
<a name="readme-top"></a>
<!--
*** Thanks for checking out the Best-README-Template. If you have a suggestion
*** that would make this better, please fork the repo and create a pull request
*** or simply open an issue with the tag "enhancement".
*** Don't forget to give the project a star!
*** Thanks again! Now go create something AMAZING! :D
-->



<!-- PROJECT SHIELDS -->
<!--
*** I'm using markdown "reference style" links for readability.
*** Reference links are enclosed in brackets [ ] instead of parentheses ( ).
*** See the bottom of this document for the declaration of the reference variables
*** for contributors-url, forks-url, etc. This is an optional, concise syntax you may use.
*** https://www.markdownguide.org/basic-syntax/#reference-style-links
-->
[![Contributors][contributors-shield]][contributors-url]
[![Forks][forks-shield]][forks-url]
[![Stargazers][stars-shield]][stars-url]
[![Issues][issues-shield]][issues-url]
[![MIT License][license-shield]][license-url]
[![LinkedIn][linkedin-shield]][linkedin-url]



<!-- PROJECT LOGO -->
<br />
<div align="center">
  <!-- <a href="https://github.com/trananh-khoa/biomus-tools">
    <img src="images/logo.png" alt="Logo" width="80" height="80">
  </a> -->

<h3 align="center">Biomedical Ultrasound Tools</h3>

  <p align="center">
    A collection of useful programs built for the Biomedical Ultrasound Laboratory at Carleton University
    <br />
    <!-- <a href="https://github.com/trananh-khoa/biomus-tools"><strong>Explore the docs »</strong></a> -->
    <!-- <br /> -->
    <br />
    <!-- <a href="https://github.com/trananh-khoa/biomus-tools">View Demo</a> -->
    <!-- · -->
    <a href="https://github.com/trananh-khoa/biomus-tools/issues">Report Bug</a>
    ·
    <a href="https://github.com/trananh-khoa/biomus-tools/issues">Request Feature</a>
  </p>
</div>



<!-- TABLE OF CONTENTS -->
<details>
  <summary>Table of Contents</summary>
  <ol>
    <li>
      <a href="#about-the-project">About The Project</a>
      <!-- <ul>
        <li><a href="#built-with">Built With</a></li>
      </ul> -->
    </li>
    <li>
      <a href="#getting-started">Getting Started</a>
      <ul>
        <li><a href="#prerequisites">Prerequisites</a></li>
        <li><a href="#installation">Installation</a></li>
      </ul>
    </li>
    <li><a href="#usage">Usage</a></li>
    <li><a href="#roadmap">Roadmap</a></li>
    <li><a href="#contributing">Contributing</a></li>
    <li><a href="#license">License</a></li>
    <li><a href="#contact">Contact</a></li>
    <!-- <li><a href="#acknowledgments">Acknowledgments</a></li> -->
  </ol>
</details>



<!-- ABOUT THE PROJECT -->
## About The Project

[![biomus-tools screenshot][biomus-tools-home]]

<p align="right">(<a href="#readme-top">back to top</a>)</p>

<!-- ### Built With

* [![Vue][Vue.js]][Vue-url]

<p align="right">(<a href="#readme-top">back to top</a>)</p> -->


<!-- GETTING STARTED -->
## Getting Started

As of right now, releases are not automatically published on the repository. Follow these instructions to get a local copy running on your machine, or to build a distributable yourself.

### Prerequisites

This is an example of how to list things you need to use the software and how to install them.
* Install [Rust and Cargo](https://doc.rust-lang.org/cargo/getting-started/installation.html)
* Install [pnpm]](https://pnpm.io/installation)

### Installation

1. Clone the repo
   ```sh
   git clone https://github.com/trananh-khoa/biomus-tools.git
   ```
2. Install packages
   ```sh
   cd biomus-tools
   pnpm install
   ```
3. Run the app in development...
   ```sh
   cargo tauri dev
   ```
4. Or build a release app bundle for your OS
   ```sh
   cargo tauri build
   ```


<p align="right">(<a href="#readme-top">back to top</a>)</p>



<!-- USAGE EXAMPLES -->
## Usage

### File Converter

Ultrasound data can come in binary file formats that are hard and/or confusing to read and use. Use the file converter to easily convert ultrasound data into file formats that are more readily used in your favourite scientific computing language.

* Only conversion from ART.LAB (.zrf) files to NumPy (.npz) is supported at this time.
* Conversion from AlazarTech (.atb) and conversion to MATLAB (.mat) is coming soon!

[![biomus-tools file converter][biomus-tools-file-converter]]

<p align="right">(<a href="#readme-top">back to top</a>)</p>

### Image Extraction (Coming Soon!)

Use biomus-tool image extraction to instantly get brightness-mode (B-mode) or motion-mode (M-mode) images from ultrasound data.


<!-- ROADMAP -->
## Roadmap

- [ ] Support reading (and writing to) other file formats
    - [ ] AlazarTech .atb
    - [ ] Video .mp4
    - [ ] MATLAB .mat
    - [ ] NumPy .npy/.npz
- [ ] Image extraction

See the [open issues](https://github.com/trananh-khoa/biomus-tools/issues) for a full list of proposed features (and known issues).

<p align="right">(<a href="#readme-top">back to top</a>)</p>



<!-- CONTRIBUTING -->
## Contributing

Contributions are what make the open source community such an amazing place to learn, inspire, and create. Any contributions you make are **greatly appreciated**.

If you have a suggestion that would make this better, please fork the repo and create a pull request. You can also simply open an issue with the tag "enhancement".
Don't forget to give the project a star! Thanks again!

1. Fork the Project
2. Create your Feature Branch (`git checkout -b feat/your-feature`)
3. Commit your Changes (`git commit -m 'feat: added some your-feature'`)
4. Push to the Branch (`git push origin feat/your-feature`)
5. Open a Pull Request

<p align="right">(<a href="#readme-top">back to top</a>)</p>



<!-- LICENSE -->
## License

Distributed under the MIT License. See `LICENSE.txt` for more information.

<p align="right">(<a href="#readme-top">back to top</a>)</p>



<!-- CONTACT -->
## Contact

Khoa Tran - trananh_khoa@outlook.com

Project Link: [https://github.com/trananh-khoa/biomus-tools](https://github.com/trananh-khoa/biomus-tools)

<p align="right">(<a href="#readme-top">back to top</a>)</p>



<!-- ACKNOWLEDGMENTS -->
<!-- ## Acknowledgments

* []()
* []()
* []()

<p align="right">(<a href="#readme-top">back to top</a>)</p> -->



<!-- MARKDOWN LINKS & IMAGES -->
<!-- https://www.markdownguide.org/basic-syntax/#reference-style-links -->
[biomus-tools-home]: docs/home.jpeg
[biomus-tools-file-converter]: docs/file-converter.jpeg
[contributors-shield]: https://img.shields.io/github/contributors/trananh-khoa/biomus-tools.svg?style=for-the-badge
[contributors-url]: https://github.com/trananh-khoa/biomus-tools/graphs/contributors
[forks-shield]: https://img.shields.io/github/forks/trananh-khoa/biomus-tools.svg?style=for-the-badge
[forks-url]: https://github.com/trananh-khoa/biomus-tools/network/members
[stars-shield]: https://img.shields.io/github/stars/trananh-khoa/biomus-tools.svg?style=for-the-badge
[stars-url]: https://github.com/trananh-khoa/biomus-tools/stargazers
[issues-shield]: https://img.shields.io/github/issues/trananh-khoa/biomus-tools.svg?style=for-the-badge
[issues-url]: https://github.com/trananh-khoa/biomus-tools/issues
[license-shield]: https://img.shields.io/github/license/trananh-khoa/biomus-tools.svg?style=for-the-badge
[license-url]: https://github.com/trananh-khoa/biomus-tools/blob/master/LICENSE
[linkedin-shield]: https://img.shields.io/badge/-LinkedIn-black.svg?style=for-the-badge&logo=linkedin&colorB=555
[linkedin-url]: https://linkedin.com/in/anh-khoa-tran
[Vue.js]: https://img.shields.io/badge/vuedotjs.svg
[Vue-url]: https://vuejs.org/