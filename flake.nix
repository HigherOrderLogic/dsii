{
  inputs = {
    nixpkgs.url = "github:NixOs/nixpkgs/nixpkgs-unstable";
    crane.url = "github:ipetkov/crane";
  };
  outputs = {
    nixpkgs,
    crane,
    ...
  }: let
    inherit (nixpkgs) lib;

    systems = lib.intersectLists lib.systems.flakeExposed lib.platforms.linux;
    forAllSystems = lib.genAttrs systems;
    nixpkgsFor = forAllSystems (system: nixpkgs.legacyPackages.${system});
  in {
    packages = forAllSystems (system: let
      pkgs = nixpkgsFor.${system};
      craneLib = crane.mkLib pkgs;

      src = craneLib.cleanCargoSource ./.;
      commonArgs = {inherit src;};
      cargoArtifacts = craneLib.buildDepsOnly commonArgs;
    in {
      default = craneLib.buildPackage (
        commonArgs // {inherit cargoArtifacts;}
      );
    });

    devShells = forAllSystems (system: let
      pkgs = nixpkgsFor.${system};
      craneLib = crane.mkLib pkgs;
    in {
      default = craneLib.devShell {};
    });
  };
}
