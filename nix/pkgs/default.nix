{ pkgs }: rec {
  emacs-i3 = pkgs.callPackage ./emacs-i3.nix { };
}
