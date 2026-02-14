{
  description = "containicus";

  inputs = {
    nixpkgs.url = "nixpkgs/nixos-25.11";
    flake-utils.url = "github:numtide/flake-utils";

	  # maybe adding some custom dependency
    #el_std_py = {
    #  url = "github:melektron/el_std_py";
    #  inputs.nixpkgs.follows = "nixpkgs";
    #  inputs.flake-utils.follows = "flake-utils";
    #};
  };

  outputs = { self, nixpkgs, flake-utils, ... }@inputs:
    # you can use something other than "eachDefaultSystem" to get even more, unconventional platforms
    # but will have to rebuild a lot of stuff
    nixpkgs.lib.recursiveUpdate (flake-utils.lib.eachDefaultSystem (system: 
      let
        pkgs = import nixpkgs { inherit system; };
      in
      {
        devShells.default = pkgs.mkShell {
          propagatedBuildInputs = with pkgs; [
            rustc
            cargo
          ];
          shellHook = ''
          '';
        };
        
        packages.containicus = pkgs.callPackage ./default.nix {
          # here we could add our custom dependency to the package
          #el_std_py = inputs.el_std_py.packages."${system}".default;
        };
        packages.default = self.packages."${system}".containicus;
        
      }
    )) {
	  # here you can add some stuff that only 
    };
}
