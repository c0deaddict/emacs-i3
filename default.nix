{ pkgs ? import <nixpkgs> { } }:

with pkgs;

rustPlatform.buildRustPackage rec {
  pname = "emacs-i3";
  version = "0.1.0";

  src = ./.;

  cargoSha256 = "sha256-5kXP5juyaCfHG5eDEdHWl2rNQ9m5zW9c3UseapkXJGg=";

  meta = with lib; {
    description = "Emacs i3 unified window management";
    homepage = "https://github.com/c0deaddict/emacs-i3";
    license = licenses.mit;
    maintainers = [ maintainers.c0deaddict ];
  };
}
