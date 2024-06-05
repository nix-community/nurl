{
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
  };

  outputs = { self, nixpkgs }:
    let
      inherit (nixpkgs.lib)
        genAttrs
        importTOML
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
      devShells = eachSystem (pkgs: {
        default = pkgs.mkShell {
          packages = self.packages.${pkgs.system}.default.runtimeInputs;
        };
      });

      formatter = eachSystem (pkgs: pkgs.nixpkgs-fmt);

      overlays.default = _: prev: {
        nurl = self.packages.${prev.system}.default;
      };

      packages = eachSystem (pkgs: {
        default = self.packages.${pkgs.system}.nurl;

        nurl = pkgs.callPackage
          (
            { darwin
            , installShellFiles
            , lib
            , makeBinaryWrapper
            , rustPlatform
            , stdenv
            , gitMinimal
            , mercurial
            , nixUnstable
            , ...
            }: rustPlatform.buildRustPackage rec {
              pname = "nurl";
              inherit (package) version;

              src = ./.;

              cargoLock = {
                allowBuiltinFetchGit = true;
                lockFile = ./Cargo.lock;
              };

              runtimeInputs = [
                gitMinimal
                mercurial
                nixUnstable
              ];

              nativeBuildInputs = [
                installShellFiles
                makeBinaryWrapper
              ];

              buildInputs = lib.optionals stdenv.isDarwin [
                darwin.apple_sdk.frameworks.Security
              ];

              # tests require internet access
              doCheck = false;

              env = {
                GEN_ARTIFACTS = "artifacts";
              };

              postInstall = ''
                wrapProgram $out/bin/nurl \
                  --prefix PATH : ${lib.makeBinPath runtimeInputs}
                installManPage artifacts/nurl.1
                installShellCompletion artifacts/nurl.{bash,fish} --zsh artifacts/_nurl
              '';

              meta = {
                inherit (package) description;
                license = lib.licenses.mpl20;
                maintainers = with lib.maintainers; [ figsoda ];
                mainProgram = "nurl";
              };
            }
          )
          { };
      });
    };
}
