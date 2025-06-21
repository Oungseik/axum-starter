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
            sqlx-cli
            cargo-tarpaulin
            otel-desktop-viewer

            # optional dependencies to make RESTapi request from Neovim
            pkg-config
            openssl
            sqlite
          ];

          # TODO update the database url
          DATABASE_URL = "sqlite:./database/expanse-tracker.sqlite";
          OTEL_EXPORTER_OTLP_ENDPOINT = "http://localhost:4317";
          OTEL_TRACES_EXPORTER = "otlp";
          OTEL_EXPORTER_OTLP_PROTOCOL = "grpc";
          STATIC_ASSETS_FOLDER = "static";
          RUSTUP_TOOLCHAIN = "stable";
        };
      }
    );
}
