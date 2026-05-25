## 📄 `CONTRIBUTING.md`

# Contributing to Shaman DE

First off, thank you for considering contributing! 🧙

We follow the **Rust Code of Conduct** – be respectful and constructive.

## How to contribute

### Reporting bugs

Open an issue on GitHub with:
- A clear title and description
- Steps to reproduce
- Expected vs actual behaviour
- Screenshots or logs (if applicable)

### Suggesting features

Open an issue tagged `enhancement`. Explain the use case and how it fits the agentic grid vision.

### Pull requests

1. **Fork the repo** and create your branch from `main`.
2. **Write tests** for new features (use `cargo test`).
3. **Run `cargo fmt`** and **`cargo clippy`** – the CI will check.
4. **Update documentation** (README, inline comments).
5. **Submit the PR** – explain what it does and link related issues.

## Development setup

```bash
git clone https://github.com/yourusername/jobshaman-grid.git
cd jobshaman-grid
cargo build
