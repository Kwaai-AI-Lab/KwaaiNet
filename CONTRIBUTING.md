# Contributing to KwaaiNet
## Building Distributed AI Infrastructure Together

Welcome to the KwaaiNet community! We're building open-source distributed AI infrastructure where users own their compute and data. This guide will help you contribute effectively to this mission.

## Development Philosophy

### "Open Architecture, Community Built"
- **Transparent Architecture**: Open technical specifications and decision-making
- **Community Implementation**: Collaborative development across all components
- **Quality Gates**: Rigorous review and integration processes ensure production readiness

### Mission-Driven Development
We're not just building software - we're democratizing AI infrastructure for humanity:
- **Long-term Vision**: Belief in distributed AI's transformative potential
- **Community Ownership**: Open-source governance and collective stewardship
- **Open Source**: MIT license ensures maximum accessibility and adoption

## How to Contribute

### 🛠 Ways to Contribute

1. **Core Development**: Work on major components (see ARCHITECTURE.md)
2. **Documentation Improvements**: Enhance guides, fix typos, add examples
3. **Bug Reports**: Submit detailed issue reports with reproduction steps
4. **Feature Development**: Propose and implement new features aligned with distributed AI vision
5. **Testing**: Help test components across different platforms and scenarios
6. **Integration Examples**: Create integration examples for storage/identity systems
7. **Community Support**: Help other contributors and users

## Development Setup

### Prerequisites
- **Rust**: Latest stable version (https://rustup.rs/)
- **Node.js**: Version 18+ for web components
- **Git**: For version control and collaboration

### Local Development Environment
```bash
# Clone the repository
git clone https://github.com/Kwaai-AI-Lab/KwaaiNet.git
cd KwaaiNet

# Set up Rust toolchain
rustup update stable

# Build the CLI
cd core && cargo build --release -p kwaainet

# Run tests
cargo test

# Build documentation
cargo doc --open
```

### Cleaning up

Build artifacts accumulate quickly (`core/target/` can reach 20 GB+). Use the cleanup script to reset to a clean state:

```bash
./scripts/clean.sh              # interactive — confirm each step
./scripts/clean.sh --all        # remove everything without prompting
./scripts/clean.sh --dry-run    # preview what would be removed
```

The script removes:
- `core/target/` — Rust build artifacts
- `/tmp/go-libp2p-daemon-*` — stale Go clone dirs from the p2pd build hook
- `/usr/local/bin/kwaainet` + `p2pd` — manually copied binaries
- `~/.cargo/bin/kwaainet` — cargo-installed binary

### Development Workflow
1. **Fork** the repository to your GitHub account
2. **Create** a feature branch from `main`
3. **Develop** your contribution with tests and documentation
4. **Test** thoroughly across target platforms
5. **Submit** pull request with detailed description
6. **Iterate** based on code review feedback

## Code Standards

### Rust Code Guidelines
```rust
// Use clear, descriptive names
pub struct SovereignAINode {
    inference_engine: CandelEngine,
    network_layer: P2PNetwork,
}

// Document all public APIs
/// Initialize a new sovereign AI node with the given configuration
/// 
/// # Arguments
/// * `config` - Node configuration including network peers and resource limits
/// 
/// # Returns
/// * `Result<SovereignAINode>` - Initialized node or configuration error
pub async fn initialize(config: NodeConfig) -> Result<Self> {
    // Implementation
}

// Use Result types for error handling
pub type KwaaiResult<T> = Result<T, KwaaiError>;

// Implement comprehensive error types
#[derive(Debug, thiserror::Error)]
pub enum KwaaiError {
    #[error("Network connection failed: {0}")]
    NetworkError(String),
    #[error("Model loading failed: {0}")]
    ModelError(String),
}
```

### JavaScript/TypeScript Guidelines
```javascript
// Use TypeScript for type safety
interface KwaaiNetConfig {
    services: ServiceConfiguration;
    privacy: PrivacySettings;
    economics: EconomicSettings;
}

// Follow modern async/await patterns
class KwaaiNet {
    async initialize(config: KwaaiNetConfig): Promise<void> {
        // Implementation with proper error handling
    }
    
    on(event: string, callback: EventCallback): void {
        // Event-driven architecture
    }
}

// Use JSDoc for documentation
/**
 * Initialize KwaaiNet with sovereign AI services
 * @param {KwaaiNetConfig} config - Configuration object
 * @returns {Promise<void>} Resolves when initialization complete
 */
```

### Testing Requirements
- **Unit Tests**: Minimum 80% code coverage
- **Integration Tests**: Cross-component functionality
- **Platform Tests**: Verify functionality across target platforms
- **Performance Tests**: Benchmark against specified requirements

### Documentation Standards
- **API Documentation**: Comprehensive documentation for all public APIs
- **Examples**: Working code examples for common use cases
- **Tutorials**: Step-by-step guides for complex integrations
- **Architecture Docs**: High-level system design and component interactions

## Code Review Process

### Pull Request Requirements
1. **Clear Description**: Explain what the PR does and why
2. **Test Coverage**: Include tests for new functionality
3. **Documentation**: Update relevant documentation
4. **Performance**: Ensure no performance regressions
5. **Security**: Security review for network-facing components

### Review Criteria
- **Functionality**: Does it work as intended?
- **Architecture**: Does it fit the overall system design?
- **Performance**: Does it meet performance requirements?
- **Security**: Are there any security vulnerabilities?
- **Maintainability**: Is the code clean and well-documented?

## Community Guidelines

### Communication Channels
- **Discord**: Real-time chat and collaboration
- **GitHub Issues**: Bug reports and feature requests
- **GitHub Discussions**: Technical discussions and Q&A
- **Community Calls**: Regular video meetings (schedule TBD)

### Code of Conduct
We are committed to providing a welcoming and inclusive environment:

- **Be respectful**: Treat all community members with respect
- **Be collaborative**: We're building something bigger together
- **Be constructive**: Focus on solutions and positive outcomes
- **Be inclusive**: Welcome people of all backgrounds and skill levels
- **Be patient**: We're all learning and growing together

### Recognition & Advancement

**Contribution Recognition**:
- **Public Attribution**: Your work in release notes and changelogs
- **Documentation Credit**: Author attribution in docs you create
- **Contributor Badge**: Recognition on GitHub and project website
- **Speaking Opportunities**: Present at conferences and events
- **Technical Blog Posts**: Featured articles on project blog

**Advancement Pathways**:
- **Core Contributor Status**: After consistent high-quality contributions
- **Module Maintainer**: Own and guide specific components
- **Technical Steering Committee**: Shape architectural decisions
- **Release Manager**: Coordinate releases and quality assurance
- **Community Lead**: Guide new contributors and build ecosystem

All recognition is merit-based and transparent. We value sustained contribution quality over quantity.

## Security

### Reporting Security Vulnerabilities
**Do NOT** report security vulnerabilities through public GitHub issues.

Instead, email security concerns to: security@kwaai.ai

Include:
- Description of the vulnerability
- Steps to reproduce
- Potential impact assessment
- Suggested fixes (if any)

We'll acknowledge receipt within 48 hours and provide regular updates.

### Security Best Practices
- Never commit secrets, API keys, or private keys
- Use proper input validation and sanitization
- Implement secure communication protocols
- Follow principle of least privilege
- Regular dependency updates and security audits

## Getting Help

### Technical Questions
1. **Search existing issues**: Your question might already be answered
2. **Check documentation**: Review architecture and API docs
3. **Ask on Discord**: Real-time help from community
4. **Create GitHub issue**: For complex questions or bug reports

### Mentorship & Support
For contributors, we provide:
- **Technical Mentors**: Experienced developers available for guidance
- **Architecture Guidance**: Regular consultation on system design
- **Career Development**: Opportunities for growth within the open-source ecosystem
- **Pair Programming**: Collaborative development sessions

### Community Resources
- **Architecture Documentation**: [ARCHITECTURE.md](./ARCHITECTURE.md)
- **Integration Examples**: [INTEGRATIONS.md](./INTEGRATIONS.md)
- **API Reference**: Generated from code documentation
- **Example Projects**: Reference implementations and tutorials

## Release Process

Only maintainers cut releases. Releases are fully automated via [cargo-dist](https://opensource.axodotdev.com/cargo-dist/) and [cargo-release](https://github.com/crate-ci/cargo-release) — a single command bumps all versions, commits, tags, and pushes. The tag triggers CI which builds all platforms, publishes binaries, updates the Homebrew formula, and publishes crates to crates.io.

All workspace crates are versioned in lockstep. Versions are centralised in `[workspace.package]` and `[workspace.dependencies]` inside `core/Cargo.toml`.

### Release tooling

Install `cargo-release` once:

```bash
cargo install cargo-release
```

### Bump the version and tag

From the `core/` directory:

```bash
cd core
cargo release X.Y.Z
```

This single command:

- Bumps `[workspace.package].version` and all internal crate version references in `[workspace.dependencies]`
- Runs `cargo test` to verify nothing is broken
- Commits with `chore: bump to vX.Y.Z`
- Creates and pushes the git tag `vX.Y.Z`

The tag push triggers `.github/workflows/release.yml`, which:
- Builds `kwaainet` for all 5 targets (macOS ARM/Intel, Linux x86_64/ARM64, Windows x86_64)
- Builds `p2pd` (Go) for each platform via `scripts/build-p2pd.sh`
- Generates SHA256-verified `.tar.xz` / `.zip` archives
- Publishes `kwaainet-installer.sh` and `kwaainet-installer.ps1`
- Pushes the Homebrew formula to `Kwaai-AI-Lab/homebrew-tap`
- Publishes all crates to crates.io in dependency order

### 3. Verify the release

- **Actions tab**: all 5 `build-local-artifacts` jobs + `build-global-artifacts` + `publish-homebrew-formula` + `publish-crates` green
- **Release page**: `.tar.xz` for each platform, `.zip` for Windows, `kwaainet-installer.sh/ps1`, `sha256.sum`
- **Installer test**: `curl --proto '=https' --tlsv1.2 -LsSf .../kwaainet-installer.sh | sh` → correct version
- **Homebrew test**: `brew upgrade kwaainet && kwaainet --version`
- **crates.io**: each crate appears at the new version under [crates.io/crates/kwaai-cli](https://crates.io/crates/kwaai-cli)

---

## License

By contributing to KwaaiNet, you agree that your contributions will be licensed under the MIT License, ensuring maximum accessibility for digital public infrastructure.

---

## Ready to Contribute?

Whether you're interested in:
- **🦀 Rust/WASM core development**
- **🌐 Web technologies and browser integration**
- **📱 Mobile application development**
- **🔗 Blockchain and identity integration**
- **🏢 Enterprise compliance and security**
- **🌱 Environmental sustainability technology**

There's a place for you in the KwaaiNet community!

**Join us in building the future of distributed AI infrastructure.**

*Together, we're democratizing AI for humanity.*