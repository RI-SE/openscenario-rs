{
  description = "Rust library for parsing and manipulating OpenSCENARIO files";

  inputs.nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";

  outputs =
    { self, nixpkgs }:
    let
      inherit (nixpkgs) lib;
      systems = [
        "x86_64-linux"
        "aarch64-linux"
        "x86_64-darwin"
        "aarch64-darwin"
      ];
      forAllSystems = f: lib.genAttrs systems (system: f nixpkgs.legacyPackages.${system});
      cargoToml = lib.importTOML ./Cargo.toml;
    in
    {
      packages = forAllSystems (pkgs: {
        default = self.packages.${pkgs.stdenv.hostPlatform.system}.openscenario-rs;

        # Library plus the `xosc-validate` and `scenario_analyzer` binaries.
        openscenario-rs = pkgs.rustPlatform.buildRustPackage {
          pname = cargoToml.package.name;
          inherit (cargoToml.package) version;
          src = lib.cleanSource ./.;
          cargoLock.lockFile = ./Cargo.lock;

          # `validation` links system libxml2 through the `libxml` crate, whose
          # build.rs finds it with pkg-config and generates bindings with bindgen.
          nativeBuildInputs = [
            pkgs.pkg-config
            pkgs.rustPlatform.bindgenHook
          ];
          buildInputs = [ pkgs.libxml2 ];

          buildFeatures = [
            "builder"
            "validation"
          ];
          checkFeatures = [
            "builder"
            "validation"
          ];
          # These read conformance/corpus/, which scripts/fetch-corpus.sh clones
          # over the network; the build sandbox has neither the corpus nor network.
          checkFlags = [
            "--skip=catalog_file_and_the_root_agree_on_every_catalog_branch_corpus_file"
            "--skip=a_catalog_document_from_the_corpus_parses_through_the_root"
          ];

          meta = {
            inherit (cargoToml.package) description homepage;
            license = lib.licenses.gpl3Only;
            mainProgram = "xosc-validate";
          };
        };
      });

      devShells = forAllSystems (pkgs: {
        default = pkgs.mkShell {
          inputsFrom = [ self.packages.${pkgs.stdenv.hostPlatform.system}.openscenario-rs ];
          packages = [
            pkgs.clippy
            pkgs.rustfmt
            pkgs.rust-analyzer
            pkgs.git
          ];
          RUST_SRC_PATH = "${pkgs.rustPlatform.rustLibSrc}";
        };
      });

      checks = forAllSystems (pkgs: {
        package = self.packages.${pkgs.stdenv.hostPlatform.system}.openscenario-rs;
      });

      formatter = forAllSystems (pkgs: pkgs.nixfmt);
    };
}
