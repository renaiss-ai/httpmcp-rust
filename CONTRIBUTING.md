# Contributing to httpmcp-rust

Thank you for your interest in contributing to httpmcp-rust! We welcome contributions from everyone.

## Getting Started

1. Fork the repository
2. Clone your fork: `git clone https://github.com/renaiss-ai/httpmcp-rust.git`
3. Create a new branch: `git checkout -b feature/your-feature-name`
4. Make your changes
5. Run tests: `cargo test`
6. Run formatting: `cargo fmt`
7. Run clippy: `cargo clippy -- -D warnings`
8. Commit your changes: `git commit -m "Add your feature"`
9. Push to your fork: `git push origin feature/your-feature-name`
10. Open a Pull Request

## Development Guidelines

### Code Style

- Follow Rust standard naming conventions
- Use `cargo fmt` to format your code
- Ensure `cargo clippy` passes without warnings
- Write clear, concise comments for complex logic
- Keep functions small and focused

### Testing

- Add tests for new functionality
- Ensure all existing tests pass
- Include integration tests for new features
- Test with `cargo test --all-features`

### Documentation

- Document all public APIs with doc comments
- Include examples in doc comments where applicable
- Update README.md if adding new features
- Keep CHANGELOG.md updated

### Commit Messages

- Use clear, descriptive commit messages
- Start with a verb in present tense (e.g., "Add", "Fix", "Update")
- Keep the first line under 50 characters
- Add detailed description if needed

Example:
```
Add support for custom middleware

- Implement middleware trait
- Add examples for custom logging
- Update documentation
```

## Pull Request Process

1. Ensure your code compiles and all tests pass
2. Update documentation as needed
3. Add an entry to CHANGELOG.md
4. Request review from maintainers
5. Address any feedback

## Code of Conduct

### Our Standards

- Be respectful and inclusive
- Welcome newcomers
- Accept constructive criticism gracefully
- Focus on what's best for the community

### Unacceptable Behavior

- Harassment or discriminatory language
- Trolling or insulting comments
- Publishing others' private information
- Other conduct which could reasonably be considered inappropriate

## Questions?

- Open an issue for questions
- Check existing issues and discussions first
- Be patient and respectful

## License

By contributing, you agree that your contributions will be licensed under the same MIT OR Apache-2.0 dual license.
