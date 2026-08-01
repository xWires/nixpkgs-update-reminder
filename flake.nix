{
  description = "Send a notification if nixpkgs hasn't been updated recently";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
  };

  outputs = { self, nixpkgs }:
    let
      forEachSystem = nixpkgs.lib.genAttrs [ "x86_64-linux" "aarch64-linux" ];
    in
    {
      packages = forEachSystem (system: {
        default = nixpkgs.legacyPackages.${system}.callPackage ./default.nix { };
      });

      apps = forEachSystem (system: {
        default = {
          type = "app";
          program = "${self.packages.${system}.default}/bin/nixpkgs-update-reminder";
        };
      });

      nixosModules.default = import ./modules/nixos.nix self;
    };
}
