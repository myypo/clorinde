{
  inputs = {
    nixpkgs.url = "nixpkgs/nixos-unstable";

    fenix.url = "github:nix-community/fenix";
    fenix.inputs.nixpkgs.follows = "nixpkgs";
  };

  outputs =
    { nixpkgs, fenix, ... }:
    let
      supportedSystems = [
        "x86_64-linux"
        "aarch64-linux"
        "x86_64-darwin"
        "aarch64-darwin"
      ];
      forEachSupportedSystem =
        f:
        nixpkgs.lib.genAttrs supportedSystems (
          system:
          f {
            pkgs = import nixpkgs {
              inherit system;
              overlays = [ fenix.overlays.default ];
            };

            rust-toolchain =
              let
                fenix' = fenix.packages.${system};
                toolchain = fenix'.complete;
              in
              fenix'.combine [
                toolchain.cargo
                toolchain.rustc
                toolchain.rust-src
                toolchain.clippy
                toolchain.rustfmt
              ];
          }
        );
    in
    {
      devShells = forEachSupportedSystem (
        { pkgs, rust-toolchain }:
        {
          default = pkgs.mkShell {
            packages = with pkgs; [
              openssl
              pkg-config
              rust-analyzer-nightly
              rust-toolchain

              postgresql
            ];
          };
        }
      );
    };
}
