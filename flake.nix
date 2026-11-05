{
  description = "wopen - workspace-aware URL opener for Sway and Hyprland";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs?ref=nixos-unstable";
  };

  outputs =
    {
      self,
      nixpkgs,
    }:
    let
      system = "x86_64-linux";
      pkgs = import nixpkgs { inherit system; };

      wopen = pkgs.rustPlatform.buildRustPackage {
        pname = "wopen";
        version = "0.1.0";
        src = ./.;

        cargoLock = {
          lockFile = ./Cargo.lock;
        };

        meta = with pkgs.lib; {
          description = "Open URLs in a browser based on workspace presence (Sway/Hyprland)";
          homepage = "https://github.com/yikesboy/wopen";
          license = licenses.mit;
          mainProgram = "wopen";
          platforms = platforms.linux;
        };
      };
    in
    {
      packages.${system} = {
        default = wopen;
        wopen = wopen;
      };

      formatter.${system} = pkgs.nixfmt-tree;

      checks.${system}.default = wopen;

      # dev shell for working on the project
      devShells.${system}.default = pkgs.mkShell {
        inputsFrom = [ wopen ];
        packages = with pkgs; [
          cargo
          rustc
          rustfmt
          clippy
          rust-analyzer
        ];
      };
    };
}
