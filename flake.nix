{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs?ref=nixos-unstable";
  };
  outputs =
    {
      self,
      nixpkgs,
    }:
    let
      systems = [
        "x86_64-linux"
        "aarch64-linux"
        "aarch64-darwin"
      ];
      forAllSystems = f: nixpkgs.lib.genAttrs systems (system: f system nixpkgs.legacyPackages.${system});
    in
    {
      packages = forAllSystems (
        system: pkgs: {
          quiztui = pkgs.callPackage ./nix/package.nix { };
          default = self.packages.${system}.quiztui;
        }
      );

      devShells = forAllSystems (
        _: pkgs: {
          default = pkgs.callPackage ./nix/shell.nix { };
        }
      );
    };
}
