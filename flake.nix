{
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
  };

  outputs = { self, nixpkgs }:
    let
      inherit (nixpkgs.lib)
        genAttrs importTOML licenses makeBinPath maintainers systems;
      inherit (importTOML (self + "/Cargo.toml")) package;
    in
    {
      formatter = genAttrs systems.flakeExposed
        (system: nixpkgs.legacyPackages.${system}.nixpkgs-fmt);

      packages = genAttrs
        [
          "aarch64-darwin"
          "aarch64-linux"
          "x86_64-darwin"
          "x86_64-linux"
        ]
        (system:
          let
            inherit (nixpkgs.legacyPackages.${system})
              gitMinimal makeWrapper mercurial nix rustPlatform;
          in
          {
            default = rustPlatform.buildRustPackage {
              pname = "nurl";
              inherit (package) version;

              src = self;

              cargoLock.lockFile = self + "/Cargo.lock";

              nativeBuildInputs = [ makeWrapper ];

              postInstall = ''
                wrapProgram $out/bin/nurl \
                  --prefix PATH : ${makeBinPath [ gitMinimal mercurial nix ]}
              '';

              meta = {
                inherit (package) description;
                license = licenses.mpl20;
                maintainers = with maintainers; [ figsoda ];
              };
            };
          });
    };
}
