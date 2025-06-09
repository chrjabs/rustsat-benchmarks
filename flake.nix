{
  description = "Dev env for RustSAT benchmarks";

  inputs = {
    nixpkgs.url = "github:chrjabs/nixpkgs/update-python-sat";
    systems.url = "github:nix-systems/default-linux";

    rust-overlay.url = "github:oxalica/rust-overlay";
    rust-overlay.inputs.nixpkgs.follows = "nixpkgs";
  };

  outputs = inputs @ {
    self,
    nixpkgs,
    systems,
    rust-overlay,
  }: let
    lib = nixpkgs.lib;
    pkgsFor = lib.genAttrs (import systems) (system: (import nixpkgs {
      inherit system;
      overlays = [(import rust-overlay)];
    }));
    forEachSystem = f: lib.genAttrs (import systems) (system: f pkgsFor.${system});
  in {
    devShells = forEachSystem (pkgs: {
      default = let
        libs = with pkgs; [openssl xz bzip2 zlib];
        python = pkgs.python3.withPackages (python-pkgs:
          with python-pkgs; [
            python-sat
          ]);
        latex = pkgs.texlive.combine {
          inherit
            (pkgs.texlive)
            scheme-small
            latexmk
            tools
            standalone
            luatex85
            pdfx
            pgf
            pgfplots
            greek-fontenc
            xmpincl
            ;
        };
      in
        pkgs.mkShell.override {stdenv = pkgs.clangStdenv;} rec {
          nativeBuildInputs = with pkgs; [
            pkg-config
            just
            cmake
            (rust-bin.fromRustupToolchainFile ./rust-toolchain.toml)
            python
            latex
            texlivePackages.pdfcrop
            poppler_utils
          ];
          buildInputs = libs;
          LD_LIBRARY_PATH = pkgs.lib.makeLibraryPath libs;
          LIBCLANG_PATH = "${pkgs.libclang.lib}/lib";
          PKG_CONFIG_PATH = "${pkgs.openssl.dev}/lib/pkgconfig/";
        };
    });
  };
}
