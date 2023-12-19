{ pkgs
, lib
, rust
}:
pkgs.clang16Stdenv.mkDerivation {
  pname = "simplicity-webide";
  version = "dev";
  src = lib.cleanSource ./.;

  buildInputs = [
    rust
    pkgs.trunk
  ];

  buildPhase = ''
    export CC_wasm32_unknown_unknown="${pkgs.llvmPackages_16.clang-unwrapped}/bin/clang-16"
    export CFLAGS_wasm32_unknown_unknown="-I ${pkgs.llvmPackages_16.libclang.lib}/lib/clang/16/include/"
    trunk build --release
    sh fix-links.sh
  '';

  installPhase = ''
    mkdir -p $out
    cp -r dist/* $out/
  '';
}
