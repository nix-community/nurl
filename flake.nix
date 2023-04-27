{
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
  };

  outputs = { self, nixpkgs }:
    let
      inherit (nixpkgs.lib)
        genAttrs
        importTOML
        licenses
        maintainers
        makeBinPath
        optionals
        sourceByRegex
        ;

      inherit (importTOML (self + "/Cargo.toml")) package;

      eachSystem = f: genAttrs
        [
          "aarch64-darwin"
          "aarch64-linux"
          "x86_64-darwin"
          "x86_64-linux"
        ]
        (system: f nixpkgs.legacyPackages.${system});
    in
    {
      formatter = eachSystem (pkgs: pkgs.nixpkgs-fmt);

      herculesCI.ciSystems = [
        "x86_64-linux"
        "aarch64-linux"
      ];

      packages = eachSystem (pkgs:
        let
          inherit (pkgs)
            darwin
            gitMinimal
            installShellFiles
            makeBinaryWrapper
            mercurial
            nixVersions
            rustPlatform
            stdenv
            ;

          src = sourceByRegex self [
            "(src|tests)(/.*)?"
            "Cargo\\.(toml|lock)"
            "build.rs"
          ];
        in
        {
          default = rustPlatform.buildRustPackage {
            pname = "nurl";
            inherit (package) version;

            inherit src;

            cargoLock = {
              allowBuiltinFetchGit = true;
              lockFile = src + "/Cargo.lock";
            };

            nativeBuildInputs = [
              installShellFiles
              makeBinaryWrapper
            ];

            buildInputs = optionals stdenv.isDarwin [
              darwin.apple_sdk.frameworks.Security
            ];

            # tests require internet access
            doCheck = false;

            env = {
              GEN_ARTIFACTS = "artifacts";
            };

            postInstall = ''
              wrapProgram $out/bin/nurl \
                --prefix PATH : ${makeBinPath [ gitMinimal mercurial nixVersions.unstable ]}
              installManPage artifacts/nurl.1
              installShellCompletion artifacts/nurl.{bash,fish} --zsh artifacts/_nurl
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
