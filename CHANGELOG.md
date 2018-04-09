# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](http://keepachangelog.com/en/1.0.0/)
and this project adheres to [Semantic Versioning](http://semver.org/spec/v2.0.0.html).


## [Unreleased]


## [0.2.3] - 2018-04-08
Thanks to the following contributors:
- [@Bond-009]

### Added
- Connection manager with reconnection
- Method to clear the current Rich Presence state

### Changed
- Move rich presence code back into *models*
- Remove command payload and add generic one
- Timestamps are now 64 bit unsigned integers instead of 32 bit ([@Bond-009]) [c:6bbc9f8]


## [0.2.2] - 2018-04-03
### Changed
- Use a default socket connection for the current platform


## [0.2.1] - 2018-04-03
### Changed
- Move common connection methods into trait


## [0.2.0] - 2018-04-02
Thanks to the following contributors:
- [@Tenrys]

### Added
- Add error type
- Add Windows support ([@Tenrys]) [c:620e9a6]

### Changed
- Convert OpCode with `try_from` instead of `try`
- Use Rust 1.25 style nested imports


## [0.1.5] - 2018-03-28
### Changed
- Opcode stored in Message is now an OpCode enum
- Rich Presence now lives in it's own submodule


## [0.1.4] - 2018-03-23
### Changed
- Opcodes are now represented as enum instead of integers


## [0.1.3] - 2018-03-23
### Added
- Contributing information

### Changed
- Use `libc::getpid` to allow builds with *stable* instead of *nightly*
- Make client struct fields private
- Make models private again and add prelude
- Connections are now using a shared Connection trait


## [0.1.2] - 2018-03-22
### Added
- Logging support


## [0.1.1] - 2018-03-22
### Changed
- Make models publicly accessible


## [0.1.0] - 2018-03-22
### Added
- Setting Rich Presence status
- Unix socket connection support