
{
  description = "Rust dev shell";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-25.11";
    fenix.url = "github:nix-community/fenix";
  };

  outputs = { self, nixpkgs, fenix }:
  let
    system = "x86_64-linux";
    pkgs = import nixpkgs { inherit system; };

    rust = with fenix.packages.${system}; combine [
            stable.toolchain
            targets.wasm32-unknown-unknown.stable.rust-std
          ];
  in {
    devShells.${system}.default = pkgs.mkShell {
      packages = [
        rust
	pkgs.llvmPackages.bintools
	pkgs.wasm-pack
        pkgs.just
        pkgs.neovim
        pkgs.bacon
        pkgs.zsh
        pkgs.cargo-tarpaulin
	pkgs.dioxus-cli
	pkgs.wasm-bindgen-cli_0_2_108
	pkgs.nodejs_24
	pkgs.binaryen

      ];

      nativeBuildInputs = [
        pkgs.pkg-config
        pkgs.gcc
      ];
	 CARGO_TARGET_WASM32_UNKNOWN_UNKNOWN_LINKER = "lld"; 
    };
  };
}

