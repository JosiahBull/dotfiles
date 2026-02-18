{
  description = "JosiahBull/dotfiles – shell environment & dev-machine setup";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-24.11";
  };

  outputs = { self, nixpkgs, ... }:
    let
      system = "x86_64-linux";
      pkgs = nixpkgs.legacyPackages.${system};

      # Minimal NixOS configuration that imports the module, used to verify
      # the module evaluates without errors.
      testSystem = nixpkgs.lib.nixosSystem {
        inherit system;
        modules = [
          self.nixosModules.default
          {
            # Minimal options required to evaluate a NixOS config
            boot.loader.grub.devices = [ "nodev" ];
            fileSystems."/" = { device = "none"; fsType = "tmpfs"; };

            dotfiles.enable = true;
            dotfiles.user = "testuser";
            users.users.testuser = {
              isNormalUser = true;
              home = "/home/testuser";
            };
          }
        ];
      };
    in
    {
      nixosModules.default = import ./nix/nixos-module.nix;

      checks.${system} = {
        # Evaluate the full NixOS system config — catches type errors,
        # missing options, broken references, etc.
        nixos-module = testSystem.config.system.build.toplevel;
      };
    };
}
