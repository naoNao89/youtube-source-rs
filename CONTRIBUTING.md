# Contributing to YouTube Source Rust

Thank you for your interest in contributing to the YouTube Source Rust project! This document provides guidelines and information for contributors.

## Table of Contents

- [Development Setup](#development-setup)
- [CI/CD Pipeline](#cicd-pipeline)
- [Testing](#testing)
- [Code Quality](#code-quality)
- [Pull Request Process](#pull-request-process)
- [Release Process](#release-process)

## Development Setup

### Prerequisites

- Rust 1.70.0 or later
- Docker and Docker Compose (for integration testing)
- Node.js 18+ (for test bot)

### Local Development

1. Clone the repository:
```bash
git clone https://github.com/lavalink-devs/youtube-source-rs.git
cd youtube-source-rs
```

2. Install dependencies:
```bash
cargo build
```

3. Run tests:
```bash
cargo test
```

4. Run benchmarks:
```bash
cargo bench
```

## CI/CD Pipeline

Our CI/CD pipeline is built with GitHub Actions and includes the following stages:

### Continuous Integration

#### Code Quality Checks (`check` job)
- **Formatting**: `cargo fmt --all -- --check`
- **Linting**: `cargo clippy --all-targets --all-features -- -D warnings`
- **Documentation**: `cargo doc --no-deps --document-private-items --all-features`

#### Security Audit (`security` job)
- **Dependency Audit**: `cargo audit`
- Runs on every push and pull request
- Checks for known security vulnerabilities

#### Multi-Platform Testing (`test` job)
- **Platforms**: Ubuntu, Windows, macOS
- **Rust Versions**: stable, beta, 1.70.0 (MSRV)
- **Features**: All feature combinations tested
- **Examples**: All examples are executed to ensure they work

#### Code Coverage (`coverage` job)
- **Tool**: cargo-tarpaulin
- **Target**: >90% code coverage
- **Reporting**: Codecov integration
- **Format**: Cobertura XML

#### Performance Benchmarks (`benchmark` job)
- **Trigger**: Only on main branch pushes
- **Tool**: Criterion.rs
- **Metrics**: Client performance, cipher operations, URL parsing

#### Build Artifacts (`build` job)
- **Targets**: Linux (x86_64, musl), Windows (x86_64), macOS (x86_64, aarch64)
- **Artifacts**: Uploaded for each platform
- **Retention**: 90 days

### Continuous Deployment

#### Release Workflow (`.github/workflows/release.yml`)

Triggered on:
- Git tags matching `v*` pattern
- Manual release creation

**Release Jobs:**
1. **Create Release**: Generates changelog and creates GitHub release
2. **Publish to crates.io**: Publishes library to Rust package registry
3. **Build Cross-Platform Binaries**: Creates binaries for all supported platforms
4. **Build Plugin JAR**: Creates Lavalink plugin JAR file

#### Dependency Updates

**Dependabot Configuration** (`.github/dependabot.yml`):
- **Cargo dependencies**: Weekly updates on Mondays
- **GitHub Actions**: Weekly updates on Mondays
- **Docker**: Weekly updates on Mondays

## Testing

### Test Categories

#### Unit Tests (`tests/unit_tests.rs`)
- Client functionality
- Manager operations
- Configuration handling
- Error scenarios
- **Coverage Target**: >95%

#### Integration Tests (`tests/integration_tests.rs`)
- End-to-end workflows
- Manager integration
- URL handling
- **Coverage Target**: >90%

#### Cipher Tests (`tests/cipher_tests.rs`)
- Signature decryption
- URL parameter handling
- Concurrent operations
- **Coverage Target**: >90%

#### Utility Tests (`tests/utils_tests.rs`)
- URL parsing and validation
- Duration formatting
- Filename sanitization
- **Coverage Target**: >95%

#### Mock Tests (`tests/mock_tests.rs`)
- YouTube API mocking
- Error response handling
- Rate limiting simulation
- **Requires**: `mock-testing` feature

#### Lavalink Integration Tests (`tests/lavalink_tests.rs`)
- Real Lavalink instance testing
- Plugin compatibility
- Performance validation
- **Requires**: `integration-tests` feature and running Lavalink

### Running Tests

```bash
# Unit and integration tests
cargo test

# With all features
cargo test --all-features

# Mock tests
cargo test --features mock-testing

# Lavalink integration tests (requires Docker)
cd test-infrastructure
./run-tests.sh

# Specific test categories
cargo test unit_tests
cargo test integration_tests
cargo test cipher_tests
```

### Lavalink Integration Testing

We provide a comprehensive Docker-based testing environment:

```bash
cd test-infrastructure

# Start test environment
./run-tests.sh start

# Run all tests
./run-tests.sh test

# Run only test bot
./run-tests.sh bot

# Run only Rust integration tests
./run-tests.sh rust

# Check status
./run-tests.sh status

# View logs
./run-tests.sh logs

# Stop environment
./run-tests.sh stop
```

**Test Environment Includes:**
- Lavalink v4 (port 2333)
- Lavalink v3 (port 2334)
- Redis (port 6379)
- Prometheus (port 9090)
- Grafana (port 3000)
- Test Bot (automated testing)

## Code Quality

### Formatting
```bash
cargo fmt
```

### Linting
```bash
cargo clippy --all-targets --all-features
```

### Documentation
```bash
cargo doc --open
```

### Security Audit
```bash
cargo audit
```

### Performance Benchmarks
```bash
cargo bench
```

## Pull Request Process

1. **Fork** the repository
2. **Create** a feature branch from `main`
3. **Make** your changes following our coding standards
4. **Add** tests for new functionality
5. **Ensure** all tests pass locally
6. **Run** code quality checks
7. **Submit** a pull request

### PR Requirements

- [ ] All tests pass
- [ ] Code coverage maintained (>90%)
- [ ] Documentation updated
- [ ] Changelog entry added (if applicable)
- [ ] No clippy warnings
- [ ] Formatted with `cargo fmt`

### Review Process

1. **Automated Checks**: CI pipeline must pass
2. **Code Review**: At least one maintainer approval
3. **Testing**: Integration tests in PR environment
4. **Merge**: Squash and merge to main

## Release Process

### Version Numbering

We follow [Semantic Versioning](https://semver.org/):
- **MAJOR**: Breaking changes
- **MINOR**: New features (backward compatible)
- **PATCH**: Bug fixes (backward compatible)

### Release Steps

1. **Update Version**: Bump version in `Cargo.toml`
2. **Update Changelog**: Add release notes to `CHANGELOG.md`
3. **Create Tag**: `git tag v1.2.3`
4. **Push Tag**: `git push origin v1.2.3`
5. **Automated Release**: GitHub Actions handles the rest

### Release Artifacts

- **Source Code**: GitHub release
- **Rust Crate**: crates.io
- **Binaries**: Cross-platform executables
- **Plugin JAR**: Lavalink plugin
- **Documentation**: GitHub Pages

## Development Guidelines

### Code Style

- Follow Rust standard formatting (`cargo fmt`)
- Use meaningful variable and function names
- Add documentation for public APIs
- Include examples in documentation

### Error Handling

- Use `Result<T, E>` for fallible operations
- Create custom error types with `thiserror`
- Provide meaningful error messages
- Log errors appropriately

### Performance

- Profile critical paths with benchmarks
- Avoid unnecessary allocations
- Use appropriate data structures
- Consider async/await for I/O operations

### Security

- Validate all inputs
- Use secure HTTP clients
- Audit dependencies regularly
- Follow OWASP guidelines

## Getting Help

- **Issues**: GitHub Issues for bugs and feature requests
- **Discussions**: GitHub Discussions for questions
- **Discord**: Lavalink Discord server
- **Documentation**: README.md and rustdoc

## License

By contributing, you agree that your contributions will be licensed under the MIT License.
