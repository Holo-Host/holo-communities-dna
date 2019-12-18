{ pkgs ? import ./pkgs.nix {} }:

with pkgs;

let
  inherit (darwin.apple_sdk.frameworks) CoreServices Security;
in

{
  hylo-holo-dnas = buildDNA {
    name = "hylo-holo-dnas";
    src = gitignoreSource ./.;

    nativeBuildInputs = [ pkgs.libiconv ]
    ++ lib.optionals stdenv.isDarwin [ CoreServices ];
  };
}
