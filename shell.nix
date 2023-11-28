{ pkgs ? import (builtins.fetchTarball {
    url = "https://github.com/NixOS/nixpkgs/archive/4ecab3273592f27479a583fb6d975d4aba3486fe.tar.gz";
    sha256 = "10wn0l08j9lgqcw8177nh2ljrnxdrpri7bp0g7nvrsn9rkawvlbf";
  }) {}
}:
pkgs.mkShell.override {
  stdenv = pkgs.clang16Stdenv;
} {
  CC_wasm32_unknown_unknown = "${pkgs.llvmPackages_16.clang-unwrapped}/bin/clang-16";
  CFLAGS_wasm32_unknown_unknown = "-I ${pkgs.llvmPackages_16.libclang.lib}/lib/clang/16/include/";
  buildInputs = with pkgs; [
    trunk
  ];
}
