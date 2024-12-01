{
  description = "an explorer for all your nix flakes written in rust";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs?ref=nixos-unstable";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = {
    self,
    nixpkgs,
    rust-overlay,
    flake-utils,
    ...
  }: let
    # Systems this will work on
    systems = [
      "x86_64-linux"
      "aarch64-linux"
      "x86_64-darwin"
      "aarch64-darwin"
    ];
  in
    flake-utils.lib.eachSystem systems (
      system: let
        overlays = [(import rust-overlay)];
        pkgs = import nixpkgs {
          inherit system overlays;
        };

        rustToolchain = pkgs.rust-bin.stable.latest.default.override {
          extensions = ["rust-src" "rust-analyzer"];
          # targets = [ "wasm32-unknown-unknown" ];
        };

      in {
        devShells.default = pkgs.mkShellNoCC {
          buildInputs = with pkgs; [
            rustToolchain
            pkg-config
            openssl.dev
            # trunk
            # wasm-bindgen-cli
            cargo-watch
            cargo-edit
            cargo-audit
            alejandra
            git
            # nodePackages.npm
          ];

          shellHook = ''
            echo "🦀 Rust ver: $(rustc --version)"
            exec ${pkgs.zsh}/bin/zsh
          '';

          # OPENSSL_NO_VENDOR = "1";
        };
      }
    );
}
