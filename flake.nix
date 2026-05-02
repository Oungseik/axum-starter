{
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
    utils.url = "github:numtide/flake-utils";
  };
  outputs =
    {
      self,
      nixpkgs,
      utils,
    }:
    utils.lib.eachDefaultSystem (
      system:
      let
        pkgs = nixpkgs.legacyPackages.${system};
      in
      {
        devShell = pkgs.mkShellNoCC {
          buildInputs = with pkgs; [
            cargo-audit
            cargo-nextest
            cargo-outdated
            cargo-tarpaulin
            sea-orm-cli
            otel-desktop-viewer

            # optional dependencies to make RESTapi request from Neovim
            pkg-config
            openssl
            sqlite
          ];

          DATABASE_URL = "sqlite:./data.db?mode=rwc";
          OTEL_EXPORTER_OTLP_ENDPOINT = "http://localhost:4318";
          OTEL_TRACES_EXPORTER = "otlp";
          OTEL_EXPORTER_OTLP_PROTOCOL = "http/protobuf";
          STATIC_ASSETS_FOLDER = "static";
          RUSTUP_TOOLCHAIN = "stable";
        };
      }
    );
}
