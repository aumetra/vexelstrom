{
  inputs = {
    flake-utils.url = "github:numtide/flake-utils";
    nixpkgs.url = "github:nixos/nixpkgs?ref=nixos-unstable";
  };

  outputs =
    {
      self,
      nixpkgs,
      flake-utils,
    }@inputs:
    flake-utils.lib.eachDefaultSystem (
      system:
      let
        pkgs = import nixpkgs {
          inherit system;
        };
      in
      {
        formatter = pkgs.nixfmt-rfc-style;

        devShells = rec {
          default = frontend;

          frontend = pkgs.mkShell {
            buildInputs = with pkgs; [
              nodejs
              nodePackages.svelte-language-server
              nodePackages.typescript-language-server
              pnpm
            ];
          };
        };
      }
    );
}
