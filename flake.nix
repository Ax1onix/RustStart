{
  description = "A very basic rust flake";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs?ref=nixos-unstable";
  };

  outputs = { self, nixpkgs }: let
    pkgs = nixpkgs.legacyPackages."x86_64-linux";
  in {
    devShells."x86_64-linux".default = pkgs.mkShell {
      buildInputs = with pkgs; [cargo rustc rustfmt rust-analyzer];
      env.RUST_SRC_PATH = "${pkgs.rust.packages.stable.rustPlatform.rustLibSrc}";
    };

    packages."x86_64-linux".rust-app = pkgs.rustPackages.buildRustPackage {
      pname = "axrust_app";  # Name of your Rust application
      version = "0.3.0";

      src = ./.;  # This points to the directory containing Cargo.toml

      # Specify the dependencies in Cargo.toml
      cargoSha256 = "sha256-...";  # You will need to calculate this after adding the crate
    };
  };
}
