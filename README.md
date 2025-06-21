# Axum Starter Project

A basic starter for Axum with SQLx and preconfigured the OpenAPI, Opentelemetry.

## Table of Contents

1. [Project Overview](#project-overview)
2. [Usage](#usage)

## Project Overview

This project is a starter template for building web applications using the following technologies:

- [**Axum**](https://github.com/tokio-rs/axum): A web framework for building robust and scalable web applications.
- [**SQLx**](https://github.com/launchbadge/sqlx): An async SQL toolkit for interacting with databases.
- [**Utoipa**](https://github.com/juhaku/utoipa): Documentation and specification for APIs.
- **OpenTelemetry**: Observability tools for monitoring and tracing.

## Usage

Change the database url in the [flake.nix](./flake.nix) or update it in the `.env` if you are not using Nix.

If you want to use Postgres, change the connection setting in [app.rs](./src/app.rs).

Configurations are loaded in the [config.rs](./src/config.rs) and share across the app.

Can collect the traces simply with [otel-desktop-viewer](https://github.com/CtrlSpice/otel-desktop-viewer).

