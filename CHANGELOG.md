# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [1.1.2] - 2021-08-31
### Changed
 - README: center screenshot

## [1.1.1] - 2021-08-23
### Changed
 - CHANGELOG: fix wrong title level in 1.1.0 release notes.
 - Documentation: reduce indent by removing unecessary `fn main()`.

## [1.1.0] - 2021-08-23
### Added
 - New ListStoreItem::new_liststore() method (and autoderive).
 - New example relm_treeview.

### Changed
 - Documentation now uses new_liststore() instead of Gladis derive.

## [1.0.2] - 2021-08-03
### Added
 - Add a simple example.

### Removed
 - Removed Rust 1.51 support because of failing CI.

## [1.0.1] - 2021-07-05
### Changed
 - Fixed wrong suggested version in README.
 - Fixed wrong examples in README.

## [1.0.0] - 2021-07-05
### Added
- Automated build and test based on Github Actions.

### Changed
 - Updated both crates to depend on gtk (gtk-rs) 0.14 (previously was: 0.4.1).
 - Improved the API to use references instead of moving the liststore each time.

## [0.1.3] - 2020-08-24
### Changed
- Updated Gladis dependency for internal tests.
- Fix doctests.
- Improved README.

## [0.1.2] - 2020-07-29
### Changed
- Fixed bad typos in README.

## [0.1.1] - 2020-07-29
### Changed
- Fixed symlinks to LICENSE-\* and README.md file in crates.

## [0.1.0] - 2020-07-29
### Added
- Initial release.
