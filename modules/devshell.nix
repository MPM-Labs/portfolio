{ inputs, ... }: {
  perSystem = { pkgs, system, ... }: 
  let
    f = with inputs.fenix.packages.${system}; combine [
      complete.toolchain
      targets.wasm32-unknown-unknown.latest.rust-std
    ];
  in {
    devShells.default = pkgs.mkShell {
      packages = with pkgs; [
        f
        leptosfmt
        cargo-leptos
        dart-sass
        wasm-bindgen-cli_0_2_117
        binaryen
        sqlx-cli
      ];
    };
  };
}
