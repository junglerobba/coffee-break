{
  lib,
  rustPlatform,
  installShellFiles,
  libiconv,
}:

rustPlatform.buildRustPackage {
  pname = "coffee-break";
  version = "0.1.0";

  src = lib.cleanSource ./.;

  cargoLock = {
    lockFile = ./Cargo.lock;
  };

  nativeBuildInputs = [
    installShellFiles
  ];

  buildInputs = [
    libiconv
  ];

  postInstall = ''
    for shell in bash fish zsh; do
      installShellCompletion --cmd coffee-break \
        --$shell <($out/bin/coffee-break --completions $shell)
    done
  '';

  meta = with lib; {
    platforms = platforms.darwin;
    mainProgram = "coffee-break";
  };
}
