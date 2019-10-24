{ pkgs ? import ./pkgs.nix {} }:

with pkgs;

let
  emacs-with-htmlize = emacsWithPackages (epkgs: with epkgs; [
    htmlize
  ]);
  inherit (darwin.apple_sdk.frameworks) CoreServices Security;
in

{
  hylo = buildDNA {
    name = "hylo";
    src = gitignoreSource ./.;

    nativeBuildInputs = []
    ++ (callPackage ./dynamodb {}).buildInputs
    ++ lib.optionals stdenv.isDarwin [ CoreServices ];
  };

  hylo-docs = stdenv.mkDerivation {
    name = "hylo-docs";
    src = gitignoreSource ./.;

    nativeBuildInputs = [ emacs-with-htmlize ];
    makeFlags = [ "doc-all" ];

    installPhase = ''
      mkdir -p $out/nix-support
      echo "doc manual $out" > $out/nix-support/hydra-build-products
      mv doc/*.html $out
    '';
  };
}
