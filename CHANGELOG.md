# Changelog

## [0.2.3] - 2025-09-07

Fixed wrong repo url in Cargo.toml
Improved error logging

## [0.2.2] - 2025-09-05

Now you can specify templates with `--template` flag. For example, `hexstack new --template full` will create a new project with Ripress and Wynd integrated.

Now `hextack new` command can take a name. `hexstack new <project-name>` will create a new project with the given name and if no name is provided, it will prompt for a name.

## [0.2.1] - 2025-09-05

## Changed

- Added a command `new` to create a new project.

### Fixed

- Fixed a bug in the `add_dependency` function that was causing the `--version` flag to be ignored.

## [0.2.0] - 2025-09-05

### Added

- Hex Stack - A simple stack to create modern backend applications that are fast and have the best in class developer experience.
