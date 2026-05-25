# felo_cli

`felo_cli` is a command-line interface tool to interact with the Felo API.

## Features

- Send queries to the Felo API.
- Supports API key authentication via flags, environment variables, or files.
- Output raw answers or full JSON responses.
- Debug mode for troubleshooting.

## Prerequisites

- [Rust](https://www.rust-lang.org/) (latest stable version recommended)
- A Felo API key

## Installation

1. Clone the repository:
   ```bash
   git clone https://github.com/jerrych0/felo_cli.git
   cd felo_cli
   ```

2. Build the project:
   ```bash
   cargo build --release
   ```

The binary will be available at `./target/release/felo_cli`.

## Usage

### Authentication
You can provide your API key in one of the following ways:
- **Environment Variable:** Set `FELO_API_KEY`.
- **Flag:** Use `--api-key <YOUR_KEY>`.
- **File:** Use `--api-key-file <PATH_TO_KEY_FILE>`.

### Commands

```bash
# Basic query
./felo_cli "What is the news today?"

# Output as JSON
./felo_cli "What is the news today?" --json

# Output raw answer
./felo_cli "What is the news today?" --raw

# Use a key file
./felo_cli "What is the news today?" --api-key-file felo.key
```

## Security Note

- **Never** commit your API key (`felo.key` or any other file containing secrets) to version control. 
- Ensure `felo.key` is listed in your `.gitignore` file.

## License

MIT License
