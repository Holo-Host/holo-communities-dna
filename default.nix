{ pkgs ? import ./pkgs.nix {}, shell ? false }:

with pkgs;

let
  inherit (darwin.apple_sdk.frameworks) CoreServices Security;
in

{
  holo-communities-dna = buildDNA {
    inherit shell;

    name = "holo-communities-dna";
    src = gitignoreSource ./.;

    nativeBuildInputs = [
      libiconv
      cmake # required by wabt
      binaryen
      wasm-gc
      wabt
    ]
    ++ lib.optionals stdenv.isDarwin [ CoreServices ];
  };
}
