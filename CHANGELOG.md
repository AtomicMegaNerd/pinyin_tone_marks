# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.2.7] - 2025-07-03

### Changed

- Use fenix for my Nix flake.
- bump rust compiler and alpine for docker.

## [0.2.6] - 2023-03-22

### Changed

- Fixed clippy warning. Also updated Rust and docker

## [0.2.5] - 2022-04-28

### Changed

- bump rust compiler and alpine for docker.

## [0.2.4] - 2022-02-02

### Changed

- Some minor tweaks based on knowledge of Rust.

## [0.2.3] - 2021-12-24

### Changed

- Use more popular and better supported library to add colors to terminal.

## [0.2.2] - 2021-12-24

### Changed

- Simplified the code and removed the env_logger library.
- Update Docker image to use Rust 1.57

## [0.2.1] - 2021-12-12

### Changed

- Fixed a few clippy warnings.

## [0.2.0] - 2021-11-21

### Changed

- Replace log4s with env_logger which is much simpler.
- Fixes for Rust 2021 and our Docker image.
- Docker image uses 1.56

## [0.1.2] - 2021-09-03

### Changed

- Updated container to rust 1.54 with Alpine 3.14.

## [0.0.1] - 2021-06-18

### Changed

- Added proper versioning to the project including the Docker container.
- Updated container to rust 1.53.
- Changed the bind mount for the Docker container.
