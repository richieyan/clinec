# clinec - AI Configuration Generator for Cline Plugins

## Overview
`clinec` is a command-line tool written in Rust that generates AI configuration files for Cline programming plugins. It helps developers quickly set up project configurations based on their technology stack.

## Installation
1. Install Rust: https://www.rust-lang.org/tools/install
2. Clone this repository:
   ```bash
   git clone https://github.com/yourusername/clinec.git
   ```
3. Build the project:
   ```bash
   cd clinec
   cargo build --release
   ```

## Usage
```bash
clinec -t <tech_stack> <project_name>
```

Supported technology stacks:
- Go
- Java
- Python

Example:
```bash
clinec -t go my_go_project
```

This will generate:
- `.clinerules` file with technology-specific rules
- `projectBrief.md` (empty)
- `.gitignore` file specific to the technology stack

## Contributing
Contributions are welcome! Please follow these steps:
1. Fork the repository
2. Create a new branch (`git checkout -b feature/YourFeatureName`)
3. Commit your changes (`git commit -am 'Add some feature'`)
4. Push to the branch (`git push origin feature/YourFeatureName`)
5. Create a new Pull Request

## License
This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
