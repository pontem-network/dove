{
  inputs = {
    fenix.url = "github:nix-community/fenix";
    naersk.url = "github:nmattia/naersk";
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    utils.url = "github:numtide/flake-utils";

    fenix.inputs.nixpkgs.follows = "nixpkgs";
    naersk.inputs.nixpkgs.follows = "nixpkgs";
  };

  outputs = { self, nixpkgs, utils, naersk, fenix }:
    utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs { inherit system; };
        fenixArch = fenix.packages.${system};
        rustTargets = fenixArch.targets;
        llvmPackages = pkgs.llvmPackages_12;

        rustToolchain = fenixArch.latest;
        rustToolchainWasm = rustTargets.wasm32-unknown-unknown.stable;
        rustToolchainWin = rustTargets.x86_64-pc-windows-msvc.latest;

        completeRustToolchain = (fenixArch.combine [
          rustToolchain.toolchain
          rustToolchainWasm.toolchain
        ]);

        naersk-lib = naersk.lib.${system}.override {
          cargo = rustToolchain.toolchain;
          rustc = rustToolchain.toolchain;
        };

      in {

        defaultPackage = naersk-lib.buildPackage (with pkgs; {
          name = "move-tools";
          src = ./.;
	        targets = [ "dove" ];
	        buildInputs = [ pkg-config openssl ];

          PROTOC = "${protobuf}/bin/protoc";
          PROTOC_INCLUDE="${protobuf}/include";
          LLVM_CONFIG_PATH="${llvmPackages.bintools}/bin/llvm-config";
          LIBCLANG_PATH="${llvmPackages.clang-unwrapped.lib}/lib";

        });

        defaultApp = utils.lib.mkApp {
            drv = self.defaultPackage."${system}";
            exePath = "/bin/dove";
        };

        devShell = with pkgs; mkShell {
          buildInputs = [
            openssl pre-commit pkg-config
            completeRustToolchain

          ];

          PROTOC = "${protobuf}/bin/protoc";
          PROTOC_INCLUDE="${protobuf}/include";
          LLVM_CONFIG_PATH="${llvmPackages.bintools}/bin/llvm-config";
          LIBCLANG_PATH="${llvmPackages.clang-unwrapped.lib}/lib";
          RUST_SRC_PATH = "${rustToolchain.rust-src}/lib/rustlib/src/rust/diemry/";

        };

      });

}
