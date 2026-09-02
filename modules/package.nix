{ inputs, ... }: {
  perSystem = { pkgs, lib, system, ... }: 
  let
    f = with inputs.fenix.packages.${system}; combine [
      complete.toolchain
      targets.wasm32-unknown-unknown.latest.rust-std
    ];
    craneLib = (inputs.crane.mkLib pkgs).overrideToolchain f;
    src = lib.cleanSourceWith {
      src = ../.;
      filter = path: type:
        (craneLib.filterCargoSources path type)
        || (lib.hasInfix "/.sqlx/" path)
        || (lib.hasInfix "/.sqlx" path)
        || (lib.hasInfix "/style/" path)
        || (lib.hasInfix "/public/" path);
    };
    cargoVendorDir = craneLib.vendorCargoDeps { inherit src; };
  in {
    packages.default = pkgs.stdenv.mkDerivation {
      name = "portfolio";
      inherit src;
      nativeBuildInputs = with pkgs; [
        f
        cargo-leptos
        dart-sass
        wasm-bindgen-cli_0_2_117
        binaryen
        makeWrapper
      ];
      meta.mainProgram = "server";

      HOME = "/build";
      SASS_PATH = "${pkgs.dart-sass}/bin/sass";

      configurePhase = ''
        export SQLX_OFFLINE=true
        runHook preConfigure
        export HOME=$(mktemp -d)
        export CARGO_HOME=$PWD/.cargo-home
        mkdir -p $CARGO_HOME
        cp ${cargoVendorDir}/config.toml $CARGO_HOME/config.toml
        runHook postConfigure
      '';

      buildPhase = ''
        runHook preBuild
        cargo leptos build --release 2>&1
        runHook postBuild
      '';

      installPhase = ''
        runHook preInstall
        mkdir -p $out/bin $out/site
        cp -r target/site/. $out/site/
        cp target/release/server $out/bin/server
        wrapProgram $out/bin/server \
          --set LEPTOS_SITE_ROOT $out/site \
          --set LEPTOS_SITE_PKG_DIR pkg
        runHook postInstall
      '';
    };
  };
}
