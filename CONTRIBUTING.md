# Contributing

The repository tooling is largely orchestrated via nix flakes, justfile, and
nushell scripts.

## Prerequisites

- [nix]
- [direnv]
- [docker]

Please review [the development shell](./scripts/flake/dev.nix) for the complete
list of tools in the shell.

## Development

Source code is located in the [src](./src) directory.

Please review the [justfile](./justfile) for the complete list of commands,
existing source code and documentation on how to work with the project.

## Pull requests

Please make sure do the following when making pull requests:

- make your changes in `src`
- modify `tests` and refine until they pass
- modify `docs` if needed
- modify `CHANGELOG.md` by adding changes in the `Unreleased` heading

## Release

For release pull requests please make sure to:

- rebase onto release branch
- modify `CHANGELOG.md` by moving `Unreleased` changes into a new release
  heading
- add an appropriate git tag

[nix]: https://nixos.org/
[direnv]: https://github.com/direnv/direnv
[docker]: https://www.nushell.sh/
