{ pkgs
, lib
, darwin
, installShellFiles
, makeBinaryWrapper
, rustPlatform
, stdenv
}:
let
  inherit (lib) makeBinPath optionals;

  runtimeInputs = with pkgs; [
    gitMinimal
    mercurial
    nixVersions.unstable
  ];

  src = lib.sourceByRegex ./. [
    "(src|tests)(/.*)?"
    "Cargo\\.(toml|lock)"
    "build.rs"
  ];
  inherit (lib.importTOML (./Cargo.toml)) package;
in
rustPlatform.buildRustPackage rec  {
  inherit src;
  inherit (package) version;
  pname = package.name;

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
    wrapProgram $out/bin/nurl --prefix PATH : ${makeBinPath runtimeInputs}
    installManPage artifacts/nurl.1
    installShellCompletion artifacts/nurl.{bash,fish} --zsh artifacts/_nurl
  '';

  meta =
    let
      inherit (lib) licenses maintainers;
    in
    {
      inherit (package) description;
      license = licenses.mpl20;
      maintainers = with maintainers; [ figsoda ];
    };
}
