<div align="center" width="800">
    <h1>rbxpm, a package manager for Roblox</h1>
</div>


* [Installation](#installation)
* [Commands](#commands)
* [File formats](#manifest-format)
* [Registries](#registries)
* [License](#license)

## About

rbxpm is a fork of rbxpm; a Roblox package manager. This fork intends to resolve common (and unresolved) pain points, and limitations, and introduce new features.

### Differences
- Support path dependencies
- Support Git dependencies
- Proper packaging yanking & types.
- Decouple the public registry scope's from GitHub users & orgs
	- This would be in line with most other package registries.
- Support a sub-set of [Cargo's workspaces](https://doc.rust-lang.org/stable/cargo/reference/workspaces.html).
	- Achieves a major missing feature from Rotriever
- Support overriding dependencies
- More feature parity to Cargo & Rotriever. (Includes all of above)
- Clean up general UX issues ([rbxpm's GitHub already has various examples](https://github.com/UpliftGames/rbxpm/issues)).
	- Support Git normally, regardless of configuration (config type, auth type, etc.)
	- UX Behavior issues
### Why?
I have started this fork, because of some of the above issues. Many of which, I continuously run into. A simple easy-to-use solution is important to me, so running various patches after installing/updating rbxpm packages is getting unreasonable.  

## Installation

### With Foreman (preferred)
TBD

### From GitHub
TBD

### From Source
It's straightforward to compile rbxpm from source. rbxpm requires Rust 1.51.0 or newer.

Clone the repository and use:

```bash
cargo install --locked --path .
```

## Commands
TBD, but commands should remain the same and be similar to Cargo's commands.

## File Formats

### Manifest Format
TBD, but will be fully backward compatible with rbxpm's and should remain similar to Cargo's and Rotriever's Manifest.

### Lockfile Format
TBD, but will likely be a different structure.

## Registries
TBD. Expect support with rbxpm's registry in some way.

## License

```
This Source Code Form is subject to the terms of the Mozilla Public
  License, v. 2.0. If a copy of the MPL was not distributed with this
  file, You can obtain one at http://mozilla.org/MPL/2.0/.

Copyright (c) 2023 MEOWSPARK LLC
```