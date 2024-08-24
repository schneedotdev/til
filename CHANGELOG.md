# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.1.4](https://github.com/schneedotdev/til/compare/v0.1.3...v0.1.4) - 2024-08-24

### Added
- separate modules, rename SearchParams to Search, and move find_by fn's within Search
- pad the amount of chars of the filepath date
- find entry by date validations

### Fixed
- by_date_range will be released at a later date
- use .to_owned()
- find_by_date -> by_date, find_by_range -> by_date_range
- remove set version

### Refactor
- move code from else block
- unnecessary Path::new()

## [0.1.3](https://github.com/schneedotdev/til/compare/v0.1.2...v0.1.3) - 2024-08-17

### Added
- update_meta function used to update any new tags provided by CL
- new error variant for parsing metadata
- added regex crate
- generate metadata for note entries
- new command `add` replacing `that`

### Docs
- information on `add` command

### Fixed
- formatting in error.rs

### Removed
- title arg has been removed

## [0.1.2](https://github.com/schneedotdev/til/compare/v0.1.1...v0.1.2) - 2024-08-17

### Added
- file-related errors

### Fixed
- remove main_tests

### Refactor
- relocate match within fmt fn

## [0.1.1](https://github.com/schneedotdev/til/compare/v0.1.0...v0.1.1) - 2024-08-16

### Added
- change name of binary

### Docs
- updated command comments
