{
  description = "Rust-based, WGPU-powered, scriptable, animatable UI library";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";

    flake-utils.url = "github:numtide/flake-utils";

    rust-overlay.url = "github:oxalica/rust-overlay";
    rust-overlay.inputs.nixpkgs.follows = "nixpkgs";
  };

  outputs =
    {
      self,
      nixpkgs,
      rust-overlay,
      flake-utils,
    }:
    flake-utils.lib.eachDefaultSystem (
      system:
      let
        pkgs = import nixpkgs {
          inherit system;
          overlays = [ rust-overlay.overlays.default ];
        };
      in
      {
        # packages.default = pkgs.callPackage ./default.nix { };

        devShells.default = pkgs.mkShell rec {
          buildInputs = with pkgs; [
            rust-bin.stable.latest.default

            wayland
            libxkbcommon

            glfw
            vulkan-headers
            vulkan-loader
            vulkan-validation-layers
            vulkan-tools
            vulkan-tools-lunarg
          ];

          LD_LIBRARY_PATH = builtins.toString (pkgs.lib.makeLibraryPath buildInputs);
          VULKAN_SDK = with pkgs; "${vulkan-headers}";
          VK_LAYER_PATH = with pkgs; "${vulkan-validation-layers}/share/vulkan/explicit_layer.d";
        };

      }
    );
}
