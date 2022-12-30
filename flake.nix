{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    flake-parts = { url = "github:hercules-ci/flake-parts"; inputs.nixpkgs-lib.follows = "nixpkgs"; };
    dream2nix = { url = "github:nix-community/dream2nix"; inputs.nixpkgs.follows = "nixpkgs"; };
  };

  outputs = inputs:
    inputs.flake-parts.lib.mkFlake { inherit inputs; } {
      systems = [ "x86_64-linux" "x86_64-darwin" "aarch64-linux" "aarch64-darwin" ];
      imports = [ inputs.dream2nix.flakeModuleBeta ];

      perSystem = { pkgs, lib, ... }: {
        dream2nix.inputs."self" = {
          source = inputs.self;
          packageOverrides."nurl".add-missing-runtime-deps = {
            nativeBuildInputs = old: old ++ [ pkgs.makeWrapper ];
            postInstall = ''
              wrapProgram $out/bin/nurl \
                --prefix PATH : ${lib.makeBinPath [ pkgs.gitMinimal pkgs.mercurial pkgs.nix ]}
            '';
          };
        };
      };
    };
}
