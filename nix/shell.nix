{
  mkShell,
  cargo,
  rustc,
  rust-analyzer,
  rustfmt,
  clippy,
  taplo,
}:
mkShell {
  name = "quiztui-dev";
  packages = [
    cargo
    rustc
    rust-analyzer

    rustfmt
    clippy
    taplo
  ];
}
