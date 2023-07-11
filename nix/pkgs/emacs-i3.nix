{ lib, fetchFromGitHub, rustPlatform }:

rustPlatform.buildRustPackage rec {
  pname = "emacs-i3";
  version = "0.1.3";

  src = lib.cleanSource ../..;

  cargoLock = {
    lockFile = ../../Cargo.lock;
    outputHashes = {
      "i3ipc-0.10.1" = "sha256-E0k5tpltTw4+Oea+47qXMtYfwpK1PEuuYEP7amjB7Ic=";
    };
  };

  meta = with lib; {
    description = "Emacs i3 unified window management";
    homepage = "https://github.com/c0deaddict/emacs-i3";
    license = licenses.mit;
    maintainers = [ maintainers.c0deaddict ];
  };
}
