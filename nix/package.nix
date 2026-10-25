{
  lib,
  rustPlatform,
}:
let
  cargoToml = (lib.importTOML ../Cargo.toml).workspace.package;
in
rustPlatform.buildRustPackage (finalAttrs: {
  pname = cargoToml.name;
  version = cargoToml.version;

  __structuredAttrs = true;

  src =
    let
      root = ../.;
    in
    lib.fileset.toSource {
      inherit root;
      fileset = lib.fileset.unions [
        (root + /src)
        (root + /Cargo.toml)
        (root + /Cargo.lock)
      ];
    };

  cargoLock.lockFile = "${finalAttrs.src}/Cargo.lock";
  enableParallelBuilding = true;

  meta = {
    description = "Quizzes in your terminal";
    homepage = "github.com/DiaxuDev/quiztui";
    license = lib.licenses.mpl20;
    mainProgram = "quiztui";
  };
})
