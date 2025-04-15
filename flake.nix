{
  description = "TALW flake";

  inputs = {
       nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
       rust-overlay.url = "github:oxalica/rust-overlay";
       flake-utils.url = "github:numtide/flake-utils";
  };

  outputs =  {
      nixpkgs,
      rust-overlay,
      flake-utils,
      ...
    }:
    flake-utils.lib.eachDefaultSystem (
      system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs {
          inherit system overlays;
        };
      in
      with pkgs;
      {
        devShells.default = mkShell {
          buildInputs =
            [
              mysql80
              nodejs
              openssl
              pkg-config
              cargo-leptos
              (rust-bin.selectLatestNightlyWith (
                toolchain:
                toolchain.default.override {
                  extensions = [
                    "rust-src"
                    "rust-analyzer"
                  ];
                  targets = [ "wasm32-unknown-unknown" ];
                }
              ))
            ];

          shellHook = ''
          '';
        };
      }
    );
}
