{ rustPlatform }:
rustPlatform.buildRustPackage {
  pname = "necktangler";
  version = "0.1.0";

  cargoLock = {
    lockFile = ./Cargo.lock;
  };

  src = ./.;

  # tools on the builder machine needed to build; e.g. pkg-config
  nativeBuildInputs = [ ];

  # native libs
  buildInputs = [ ];
}
