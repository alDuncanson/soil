{
  description = "Soil - A CLI for file system operations with nature-themed naming";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, rust-overlay, flake-utils }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs {
          inherit system overlays;
        };

        # Use Rust 2024 edition compatible toolchain
        rustToolchain = pkgs.rust-bin.stable.latest.default.override {
          extensions = [ "rust-src" "rustfmt" "clippy" ];
        };

        # Common inputs for both development and building
        commonInputs = with pkgs; [
          pkg-config
          openssl
        ];

        # Build the Rust package
        soilPackage = pkgs.rustPlatform.buildRustPackage {
          pname = "soil";
          version = "0.1.0";

          src = ./.;

          cargoLock = {
            lockFile = ./Cargo.lock;
          };

          nativeBuildInputs = commonInputs ++ [ rustToolchain ];

          meta = with pkgs.lib; {
            description = "A CLI for file system operations with nature-themed naming";
            homepage = "https://github.com/yourusername/soil"; # Update with actual repo
            license = licenses.mit; # Update with actual license
            maintainers = [ ];
            platforms = platforms.unix;
          };
        };

      in
      {
        # The default package (for `nix build`)
        packages = {
          default = soilPackage;
          soil = soilPackage;
        };

        # Development environment (for `nix develop`)
        devShells.default = pkgs.mkShell {
          buildInputs = commonInputs ++ [
            rustToolchain

            # Additional development tools
            pkgs.cargo-watch
            pkgs.cargo-edit
            pkgs.cargo-audit
            pkgs.bacon

            # Documentation tools
            pkgs.mdbook

            # Git and general utilities
            pkgs.git
            pkgs.fd
            pkgs.ripgrep
          ];

          # Environment variables
          RUST_SRC_PATH = "${rustToolchain}/lib/rustlib/src/rust/library";
          RUST_BACKTRACE = "1";

          shellHook = ''
            echo "🌱 Welcome to the Soil development environment!"
            echo ""
            echo "Available commands:"
            echo "  cargo build          - Build the project"
            echo "  cargo run -- <args>  - Run soil with arguments"
            echo "  cargo test           - Run tests"
            echo "  cargo watch -x run   - Watch for changes and rebuild"
            echo "  bacon                - Interactive build watcher"
            echo ""
            echo "Example usage:"
            echo "  cargo run -- survey ."
            echo "  cargo run -- trace ./some/path"
            echo ""
          '';
        };

        # Apps (for `nix run`)
        apps = {
          default = {
            type = "app";
            program = "${soilPackage}/bin/soil";
            meta = {
              description = "Run the soil CLI";
            };
          };

          soil = {
            type = "app";
            program = "${soilPackage}/bin/soil";
            meta = {
              description = "Run the soil CLI";
            };
          };

          # Convenience apps for common operations
          dev = {
            type = "app";
            program = toString (pkgs.writeShellScript "soil-dev" ''
              exec ${pkgs.cargo-watch}/bin/cargo-watch -x "run -- $@"
            '');
            meta = {
              description = "Start development mode with file watching";
            };
          };

          test = {
            type = "app";
            program = toString (pkgs.writeShellScript "soil-test" ''
              exec ${pkgs.cargo}/bin/cargo test
            '');
            meta = {
              description = "Run the test suite";
            };
          };
        };

        # Formatter (for `nix fmt`)
        formatter = pkgs.nixpkgs-fmt;

        # Checks (for `nix flake check`)
        checks = {
          build = soilPackage;
        };
      }
    );
}
