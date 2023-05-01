{
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
  };

  outputs = { self, nixpkgs }:
    let
      eachSystem = f: nixpkgs.lib.genAttrs
        [
          "aarch64-darwin"
          "aarch64-linux"
          "x86_64-darwin"
          "x86_64-linux"
        ]
        (system: f (
          import nixpkgs {
            inherit system;
            overlays = [
              self.overlays.default
            ];
          }
        ));

      runtimeInputs = pkgs:
        with pkgs; [
          gitMinimal
          mercurial
          nixVersions.unstable
        ];
    in
    {
      overlays.default = import ./overlay.nix;

      devShells = eachSystem (pkgs: {
        default = pkgs.mkShell {
          packages = runtimeInputs pkgs;
        };
      });

      formatter = eachSystem (pkgs: pkgs.nixpkgs-fmt);

      herculesCI.ciSystems = [
        "x86_64-linux"
        "aarch64-linux"
      ];

      packages = eachSystem (pkgs: {
        default = pkgs.nurl;
      });
    };
}
