# Contributing to LEAN CHAIN

Thank you for your interest in contributing to LEAN CHAIN! This document provides guidelines and instructions for contributing.

## Code of Conduct

Be respectful and inclusive. We welcome contributions from everyone.

## How to Contribute

### Reporting Bugs

1. Check if the bug has already been reported in Issues
2. If not, create a new issue with:
   - Clear description of the bug
   - Steps to reproduce
   - Expected vs actual behavior
   - System information (OS, Rust version, etc.)

### Suggesting Features

1. Check if the feature has been suggested
2. Create a new issue with:
   - Clear description of the feature
   - Use cases and benefits
   - Potential implementation approach

### Pull Requests

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Make your changes
4. Write or update tests
5. Ensure all tests pass (`cargo test --all`)
6. Run the formatter (`cargo fmt`)
7. Run the linter (`cargo clippy`)
8. Commit your changes with clear messages
9. Push to your fork
10. Open a Pull Request

## Development Setup

### Prerequisites

- Rust 1.75 or later
- Git

### Building

```bash
# Clone the repository
git clone https://github.com/dataswarmproject/lean.git
cd lean

# Build all components
cargo build

# Run tests
cargo test --all

# Run specific module tests
cargo test -p lean-crypto
```

### Code Style

- Follow Rust standard style (`cargo fmt`)
- Use `cargo clippy` to catch common mistakes
- Write clear, self-documenting code
- Add comments for complex logic
- Write tests for new functionality

### Testing

- Write unit tests in the same file as the code
- Write integration tests in `tests/` directory
- Aim for high test coverage (>80%)
- Test edge cases and error conditions

### Commit Messages

Follow conventional commits format:

```
type(scope): description

[optional body]

[optional footer]
```

Types:
- `feat`: New feature
- `fix`: Bug fix
- `docs`: Documentation changes
- `style`: Code style changes (formatting)
- `refactor`: Code refactoring
- `test`: Adding or updating tests
- `chore`: Maintenance tasks

Example:
```
feat(consensus): implement BFT voting mechanism

Add the prevote and precommit phases for BFT consensus.
Includes validator signature aggregation.

Closes #123
```

## Project Structure

```
lean-chain/
├── core/           # Core blockchain modules
├── network/        # Networking layer
├── vm/             # Virtual machine
├── enterprise/     # Enterprise features
├── defi/           # DeFi primitives
├── bridge/         # Cross-chain bridges
├── governance/     # Governance system
├── cli/            # Command-line tools
└── node/           # Node implementations
```

## Review Process

1. PRs require at least one approval
2. All tests must pass
3. Code must pass formatting and linting
4. Documentation must be updated
5. Breaking changes require discussion

## Getting Help

- Join our [Discord server](https://discord.gg/leanchain)
- Ask questions in GitHub Discussions
- Check the [documentation](docs/)

## License

By contributing, you agree that your contributions will be licensed under both MIT and Apache-2.0 licenses.

---

Thank you for contributing to LEAN CHAIN! 🚀
