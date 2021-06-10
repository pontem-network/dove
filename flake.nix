{

  inputs = {
    fenix = {
      url = "github:nix-community/fenix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    mozpkgs = {
      url = "github:mozilla/nixpkgs-mozilla";
      flake = false;
    };
    crate2nix = {
      url = "github:kolloch/crate2nix";
      flake = false;
    };
    naersk.url = "github:nmattia/naersk/master";
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, utils, naersk, fenix, mozpkgs, crate2nix }:
    utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs { inherit system; };
        fenixArch = fenix.packages.${system};
        rustToolchain = fenixArch.latest;
        rustTargets = fenixArch.targets;
        llvmPackages = pkgs.llvmPackages_12;

        naersk-lib = naersk.lib.${system}.override {
          inherit (rustToolchain) cargo rustc;
        };

      in {

        defaultPackage =
        let
          pkgs = import nixpkgs {
            inherit system;
            overlays = [
              (import mozpkgs)
              (super: self: {
                rustc = super.latest.rustChannels.stable.rust;
                inherit (super.latest.rustChannels.stable) cargo rust-fmt rust-std clippy;
              })
              (super: self: {

              })
            ];
          };
          customBuildRustCrateForPkgs = pkgs: pkgs.buildRustCrate.override {
            defaultCrateOverrides = pkgs.defaultCrateOverrides // {
              libp2p-core = builtins.trace "LIBP2P-CORE OVERRIDE" (attrs: with pkgs; {
                buildInputs = [
                  protobuf openssl pre-commit pkgconfig
                  llvmPackages.libclang.out
                ];
                prePatchPhase = ''
                  echo PREPATCH
                '';
                # SKIP_WASM_BUILD = 1;
                PROTOC = "${protobuf}/bin/protoc";
                PROTOC_INCLUDE="${protobuf}/include";
                LLVM_CONFIG_PATH="${llvmPackages.bintools}/bin/llvm-config";
                LIBCLANG_PATH="${llvmPackages_11.clang-unwrapped.lib}/lib";
                RUST_SRC_PATH = "${rustToolchain.rust-src}/lib/rustlib/src/rust/library/";
              });
            };
          };
          crate = import ./Cargo.nix {
            inherit pkgs;
            buildRustCrateForPkgs = customBuildRustCrateForPkgs;
          };
        in crate.workspaceMembers.dove.build;
        # naersk-lib.buildPackage {
        #   root = ./.;
        #   src = ./dove;
        #   targets = [ "dove" ];
        #   release = false;
        # };

        defaultApp = utils.mkApp {
            drv = self.defaultPackage."${system}";
        };

        devShell =
        let
        c2n = pkgs.callPackage crate2nix {};
        in
        with pkgs; mkShell {
          buildInputs = [
            protobuf openssl pre-commit pkgconfig
            llvmPackages.libclang.out c2n

            (fenixArch.combine [
		rustToolchain.toolchain
                rustTargets.wasm32-unknown-unknown.stable.completeToolchain
              ]
            )

          ];

          SKIP_WASM_BUILD = 1;
          PROTOC = "${protobuf}/bin/protoc";
          PROTOC_INCLUDE="${protobuf}/include";
          LLVM_CONFIG_PATH="${llvmPackages.bintools}/bin/llvm-config";
          LIBCLANG_PATH="${llvmPackages_11.clang-unwrapped.lib}/lib";
          RUST_SRC_PATH = "${rustToolchain.rust-src}/lib/rustlib/src/rust/library/";
          # RUST_SRC_PATH = rustPlatform.rustLibSrc;ca

        };

      });

}
