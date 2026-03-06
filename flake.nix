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
        pkgs.just
        pkgs.neovim
        pkgs.bacon
        pkgs.openssl
      ];

      nativeBuildInputs = [
        pkgs.pkg-config         
	pkgs.gcc
      ];

	shellHook = ''
        export PATH="$HOME/.cargo/bin:$PATH"
      '';

      OPENSSL_DIR = "${pkgs.openssl.dev}";
      OPENSSL_LIB_DIR = "${pkgs.openssl.out}/lib";
      OPENSSL_INCLUDE_DIR = "${pkgs.openssl.dev}/include";

      CARGO_TARGET_WASM32_UNKNOWN_UNKNOWN_LINKER = "lld"; 
    };
  };
}
