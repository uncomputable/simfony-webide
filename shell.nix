{ pkgs ? import (builtins.fetchTarball {
    url = "https://github.com/NixOS/nixpkgs/archive/057f9aecfb71c4437d2b27d3323df7f93c010b7e.tar.gz";
    sha256 = "1ndiv385w1qyb3b18vw13991fzb9wg4cl21wglk89grsfsnra41k";
  }) {
    overlays = [
      (import (fetchTarball "https://github.com/oxalica/rust-overlay/archive/master.tar.gz"))
    ];
  }
}:
let
  targets = [
    "wasm32-unknown-unknown"
  ];
  rust = with pkgs; [
    (
      rust-bin.stable.latest.default.override {
        inherit targets;
      }
    )
  ];
  leptos = with pkgs; [
    trunk
  ];
  wasm-tests = with pkgs; [
    wasm-pack
    wasm-bindgen-cli
    nodejs
  ];
in
  pkgs.mkShell.override {
    stdenv = pkgs.clang16Stdenv;
  } {
    CC_wasm32_unknown_unknown = "${pkgs.llvmPackages_16.clang-unwrapped}/bin/clang-16";
    CFLAGS_wasm32_unknown_unknown = "-I ${pkgs.llvmPackages_16.libclang.lib}/lib/clang/16/include/";
    buildInputs = rust ++ leptos ++ wasm-tests;
  }
