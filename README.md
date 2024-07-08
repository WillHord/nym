<!-- PROJECT LOGO -->
<div align="center">
  <!-- TODO: Add an image here -->
  <!-- <a href="https://github.com/WillHord/nym"> -->
  <!--   <img src="images/logo.png" alt="Logo" width="80" height="80"> -->
  <!-- </a> -->

<h2 align="center">Pseudo-Nym</h2>

  <p align="center">
    A light-weight alias manager for your unix shell
    <br />
    <a href="https://github.com/WillHord/nym/issues/new?labels=bug&template=bug-report---.md">Report Bug</a>
    ·
    <a href="https://github.com/WillHord/nym/issues/new?labels=enhancement&template=feature-request---.md">Request Feature</a>
  </p>

[![Issues][issues-shield]][issues-url]
[![GNU GPL v3 License][license-shield]][license-url]
![Rust](https://img.shields.io/badge/rust-%23000000.svg?style=for-the-badge&logo=rust&logoColor=white)
[![Tests](https://github.com/WillHord/nym/actions/workflows/rust.yml/badge.svg)](https://github.com/WillHord/nym/actions/workflows/rust.yml)

</div>

<!-- TABLE OF CONTENTS -->
<details>
  <summary>Table of Contents</summary>
  <ol>
    <li>
      <a href="#about-the-project">About The Project</a>
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
    <!-- <li><a href="#contact">Contact</a></li> -->
    <!-- <li><a href="#acknowledgments">Acknowledgments</a></li> -->
  </ol>
</details>

<!-- ABOUT THE PROJECT -->

## About The Project

Pseudo-Nym is a small but powerful alias manager for Unix-based operating systems built in Rust. It was created to simplify and streamline your shell experience by providing a quick and easy way to create, manage, and document your shell aliases. Nym allows the user to create an alias and its description with one command, toggle it on and off, list all aliases, and more.

_Currently Nym has only been tested on **zsh** and **bash** shell environments._

<!-- GETTING STARTED -->

## Getting Started

To build and install nym follow these simple steps.

### Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) and Cargo

  ```sh
  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
  ```

### Installation

1. Clone the repo

   ```sh
   git clone https://github.com/WillHord/nym.git
   ```

2. Build nym and move binary to bin

   ```sh
   cargo build --release
   cp target/release/nym /usr/local/bin/nym
   ```

3. Run install command with shell profile

   ```sh
   nym install <path_to_shell_profile>
   ```

<!-- USAGE EXAMPLES -->

## Usage

- Add an alias

  ```sh
  nym add <alias> <description>
  # Eample
  nym add example="echo 'testing'" "This is an example alias"
  ```

- Toggle alias

  ```sh
  nym toggle <alias_name>
  # Eample
  nym toggle example
  ```

- Rename alias

  ```sh
  nym rename <old_alias_name> <new_alias_name>
  # Eample
  nym rename example example2
  ```

- Remove alias

  ```sh
  nym rm <alias_name>
  # Eample
  nym rm example
  ```

- View alias description

  ```sh
  nym man <alias_name>
  # Eample
  nym man example
  ```

- List all aliases

  ```sh
  nym list <optional flags>
  # Eamples
  nym list
  nym list --disabled
  ```

- Sync aliases

  ```sh
  # The Sync command finds discrepancies between the alias file and the json config file and updates accordingly
  nym sync
  ```

<!-- ROADMAP -->

## Roadmap

- [x] Alias manager interface (allow for user to toggle, add, and delete all within one command)
- [x] Rename command for aliases
- [ ] Manage Scripts as well as Aliases
- [ ] Better installation (brew, cargo, packman, etc.)
- [ ] Test on other shell environments (other than bash, and zsh)
- [ ] Download aliases from web (github repos)
- [ ] Manage more parts of shell env
  - [ ] Api keys, secrets, etc
- [ ] Aliases groups (enable sorting aliases and toggling in groups)

See the [open issues](https://github.com/WillHord/nym/issues) for a full list of proposed features and known issues.

<!-- CONTRIBUTING -->

## Contributing

If you have a suggestion that would make this better, please fork the repo and create a pull request. You can also simply open an issue with the tag "enhancement".

1. Fork the Project
2. Create your Feature Branch (`git checkout -b feature/AmazingFeature`)
3. Commit your Changes (`git commit -m 'Add some AmazingFeature'`)
4. Push to the Branch (`git push origin feature/AmazingFeature`)
5. Open a Pull Request

<!-- LICENSE -->

## License

Distributed under the GNU GPL v3 License. See [LICENSE](https://github.com/WillHord/nym/blob/main/LICENSE) for more information.

<!-- MARKDOWN LINKS & IMAGES -->
<!-- https://www.markdownguide.org/basic-syntax/#reference-style-links -->

[issues-shield]: https://img.shields.io/github/issues/WillHord/nym.svg?style=for-the-badge
[issues-url]: https://github.com/WillHord/nym/issues
[license-shield]: https://img.shields.io/github/license/github_username/repo_name.svg?style=for-the-badge
[license-url]: https://github.com/github_username/repo_name/blob/master/LICENSE.txt
