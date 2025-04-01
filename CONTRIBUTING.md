# Contributing to DotIgnore

Thank you for your interest in contributing to DotIgnore! This document provides guidelines and instructions for contributing to this project.

## Code of Conduct

By participating in this project, you agree to abide by our Code of Conduct. Please be respectful and considerate of others.

## How to Contribute

There are many ways to contribute to DotIgnore:

1. **Reporting Bugs**: If you find a bug, please create an issue describing the problem, including steps to reproduce it.
2. **Suggesting Features**: Have an idea for a new feature? Open an issue to discuss it.
3. **Improving Documentation**: Documentation improvements are always welcome, whether it's fixing typos or adding new content.
4. **Submitting Code**: Contributions to the codebase are greatly appreciated.

## Development Setup

1. **Fork the Repository**: Start by forking the repository on GitHub.
2. **Clone the Fork**: Clone your fork locally.
   ```bash
   git clone https://github.com/YOUR-USERNAME/DotIgnore.git
   cd DotIgnore
   ```
3. **Install Dependencies**: Make sure Rust is installed (1.70 or higher).
   ```bash
   rustup update stable
   ```
4. **Build the Project**:
   ```bash
   cargo build
   ```
5. **Run Tests**:
   ```bash
   cargo test
   ```

## Pull Request Process

1. **Create a Branch**: Create a branch for your changes.
   ```bash
   git checkout -b feature/your-feature
   ```
2. **Make Your Changes**: Implement your changes, following the code style of the project.
3. **Write Tests**: Add tests for your changes to ensure they work correctly.
4. **Document Your Changes**: Update the documentation if necessary.
5. **Commit Your Changes**: Use clear commit messages.
   ```bash
   git commit -m "Add feature X"
   ```
6. **Push to Your Fork**:
   ```bash
   git push origin feature/your-feature
   ```
7. **Submit a Pull Request**: Go to GitHub and submit a pull request to the main repository.

## Code Style

- Follow Rust's standard code style and formatting.
- Use `cargo fmt` to format your code before committing.
- Run `cargo clippy` to check for common mistakes and improve your code.

## Documentation Standards

- Keep documentation up to date with code changes.
- Document public APIs with doc comments.
- All documentation is available in both English and Spanish.

## License

By contributing to DotIgnore, you agree that your contributions will be licensed under the project's MIT License.

Thank you for contributing to DotIgnore!
