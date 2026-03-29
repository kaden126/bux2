# `bux2`
A refreshingly simple **b**uild2 **ux**.

## Overview
`bux2` is a lightweight wrapper around the [`build2`](https://build2.org) build toolchain for C++.

Configuration is done through a single `bux2.toml` file that maps directly onto `build2` files. 
`bux2` also provides a command line interface to run `build2` commands. 

For the best experience, `bux2.toml` and the `bux2` CLI are used together, but they don't depend on each other, and you may use a combination of traditional `build2` and `bux2` tools.

## Example
The `bux2` cli offers a `new` subcommand for creating new projects.
```sh
bux2 new example-project
```

The new project contains a source directory, a build directory, and a `bux2.toml` file.

Adjust the `[profile.debug]` and `[profile.release]` tables to your liking, then run:
```sh
bux2 init --profile debug
bux2 init --profile release
```
To initialize the configurations.

Then, build with:
```sh
bux2 build --profile debug
```
## Dependencies
Adding dependencies is as simple as adding a key to `bux2.toml`.

### cppget.org (remote) dependencies
Dependencies from the `build2` package repository are added like so:
```toml
[dependencies]
channels = ["stable"]
fmt = { version = "10.2.1", targets = ["fmt"] }
```
The special `channel` key controls what cppget repositories to download from.

The potential values are as follows:
| name | url | category |
| --- | --- | --- |
| "stable" | https://pkg.cppget.org/1/stable/ | Stable, maintained, reviewed packages |
| "testing" | https://pkg.cppget.org/1/testing/ | Newer, potentially unreviewed packages
| "legacy" | https://pkg.cppget.org/1/legacy/ | Old packages, kept for backwards compatibility.
| "alpha" | https://pkg.cppget.org/1/alpha/ | Newest available packages, very unstable.
| "beta" | https://pkg.cppget.org/1/beta/ | More refined, polished packages compared to "alpha"

### Version control (github) dependencies
Dependencies fetched from github or other source repositores are not recomended unless you are sure they provide build2 metadata. cppget packages always do.

If you are sure the repository provides the required metadata, add it like so:
```toml
[dependencies]
libawesome = { repo = "https://source/someone/libawesome", version = "1.0.0", targets = ["awesome"] }
```
### Local dependencies (on your machine)
Libraries you write and have on your machine are imported like so: 
```toml
[dependencies]
libawesome = { path = "/Users/user/c++/libawesome", version = "1.0.0", targets = ["awesome"] }
```
## Roadmap
`bux2` is a work-in-progress, and is also limited by `build2`'s progress and documentation. Some planned features:

- [ ] Support for `build2` native testing via `testscript`.
- [ ] Build-time dependencies. Currently can be specified, but don't work as intended.
- [ ] Advanced project templates.

The following features will be implemented as `build2` makes them available. 
- [ ] `compile_commands.json` generation. Supposed to be available in the next `build2` release!
