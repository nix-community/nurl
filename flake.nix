{
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
  };

  outputs = { self, nixpkgs }:
    let
      inherit (builtins) path;
      inherit (nixpkgs.lib)
        genAttrs importTOML licenses makeBinPath maintainers optionals sourceByRegex;
      inherit (importTOML (self + "/Cargo.toml")) package;

      forEachSystem = genAttrs [
        "aarch64-darwin"
        "aarch64-linux"
        "x86_64-darwin"
        "x86_64-linux"
      ];
    in
    {
      formatter = forEachSystem
        (system: nixpkgs.legacyPackages.${system}.nixpkgs-fmt);

      herculesCI.ciSystems = [
        "x86_64-linux"
        "aarch64-linux"
      ];

      packages = forEachSystem (system:
        let
          inherit (nixpkgs.legacyPackages.${system})
            darwin gitMinimal installShellFiles makeWrapper mercurial nix rustPlatform stdenv;
        in
        {
          default = rustPlatform.buildRustPackage {
            pname = "nurl";
            inherit (package) version;

            src = sourceByRegex self [
              "(src|tests)(/.*)?"
              "Cargo\\.(toml|lock)"
              "build.rs"
            ];

            cargoLock = {
              allowBuiltinFetchGit = true;
              lockFile = path {
                path = self + "/Cargo.lock";
              };
            };

            nativeBuildInputs = [
              installShellFiles
              makeWrapper
            ];

            buildInputs = optionals stdenv.isDarwin [
              darwin.apple_sdk.frameworks.Security
            ];

            # tests require internet access
            doCheck = false;

            postInstall = ''
              wrapProgram $out/bin/nurl \
                --prefix PATH : ${makeBinPath [ gitMinimal mercurial nix ]}
              installManPage artifacts/nurl.1
              installShellCompletion artifacts/nurl.{bash,fish} --zsh artifacts/_nurl
            '';

            GEN_ARTIFACTS = "artifacts";

            meta = {
              inherit (package) description;
              license = licenses.mpl20;
              maintainers = with maintainers; [ figsoda ];
            };
          };
        });
    };
}
