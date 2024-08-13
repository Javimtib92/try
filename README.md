## Papyrust

Papyrust is a command-line interface (CLI) tool to simplify and automate a variety of tasks that are otherwise tedious and time-consuming.

## Code style

[![The Rust Style Guide](https://img.shields.io/badge/code%20style-standard-brightgreen.svg?style=flat)](https://doc.rust-lang.org/nightly/style-guide/index.html)

## Tech/framework used

<b>Built with</b>
- [Clap](https://docs.rs/clap/latest/clap/)

## Installation

### Pre-Built Binaries

Pre-built binaries for Linux, MacOS, and Windows can be found on [the releases page](https://github.com/casey/just/releases). Download the binary for your platform, and place it in a directory that is included in your system's PATH environment variable for easy access from the terminal.

### Building from source

If you'd prefer to build Papyrust from source, ensure you have Rust installed on your machine. Clone the repository and run the following commands:

```bash
git clone https://github.com/Javimtib92/papyrust
cd papyrust
cargo build --release
```

This will generate a binary in the target/release directory, which you can then move to your PATH.

## How to use?

Using Papyrust is straightforward. After installation, simply invoke papyrust from your terminal, followed by the command you wish to execute. For example:

```bash
papyrust your-command
```

For a full list of available commands and their options, you can use the help flag:


```bash
papyrust --help
```

## License
Papyrust is licensed under the MIT License. See the [LICENSE](./LICENSE) file for more details.

MIT © [Javier Muñoz Tous](https://github.com/Javimtib92)
