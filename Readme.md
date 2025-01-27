# FileForge

FileForge is a command-line tool for generating some configuration code for deploying application with docker, docker compose and ansible.

## Usage

### Installation

# Windows Installation

Just run the following two commands with Powershell (not CMD)

```shell
Set-ExecutionPolicy RemoteSigned -Scope CurrentUser
[System.Net.WebClient]::new().DownloadString('https://raw.githubusercontent.com/theprantadutta/fileforge/master/install.ps1') | iex
```

# Linux Installation

```shell
curl -L https://github.com/theprantadutta/fileforge/raw/master/install.sh | bash
```

To directy use FileForge, make sure you have Rust installed. Then, you can build the project using the following command:

```bash
cargo build --release
```

This will create the executable in the `target/release` directory.

### Commands

FileForge supports the following commands:

#### Initialize Configuration

Creates a configuration file at the root of this directory

```bash
fileforge init
```

#### Generate Files and Folders

Generates Dockerfile, compose and ansible files for safely deploying application with gitlab CI/CD.

```bash
fileforge generate
```

## Contributing

Feel free to contribute to FileForge by opening issues or submitting pull requests. Your feedback and improvements are highly appreciated.

## License

This project is licensed under the Apache License - see the [LICENSE](LICENSE) file for details.
