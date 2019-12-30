{ pkgs ? import ./pkgs.nix {} }:

with pkgs;

let
  inherit (darwin.apple_sdk.frameworks) CoreServices Security;
in

{
  holo-communities-dna = buildDNA {
    name = "holo-communities-dna";
    src = gitignoreSource ./.;

    nativeBuildInputs = [ pkgs.libiconv ]
    ++ lib.optionals stdenv.isDarwin [ CoreServices ];
  };
}
