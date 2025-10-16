<div align="center">

# soil

Ergonomic filesystem API and a tiny CLI.

[![CI](https://github.com/alDuncanson/soil/actions/workflows/ci.yml/badge.svg?branch=main)](https://github.com/alDuncanson/soil/actions/workflows/ci.yml)
[![Release](https://img.shields.io/github/v/release/alDuncanson/soil?sort=semver)](https://github.com/alDuncanson/soil/releases)
[![Rust Edition](https://img.shields.io/badge/edition-2024-black?logo=rust)](https://doc.rust-lang.org/edition-guide/rust-2024/)
[![Nix Flake](https://img.shields.io/badge/Nix-flake-5277C3?logo=nixos&logoColor=white)](flake.nix)

</div>

> Note: This is an experimental project I built to learn Rust—expect rough edges.

## documentation

Generate and open the API docs in your browser:

```sh
cargo doc --no-deps --open
```

If you use Nix, you can do the same inside the dev shell:

```sh
nix develop
cargo doc --no-deps --open
```

## nix flake

This repo ships a development flake (`flake.nix`). Handy commands:

- Enter dev shell with Rust toolchain and utilities: `nix develop`
- Run the CLI via flake app: `nix run .#soil -- --help` (or simply `nix run` for the default app)
- Build with flake: `nix build`
