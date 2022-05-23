# Aftman
Aftman is a toolchain manager. It lets you install command line tools 

## Installation
To install Aftman, you can build it from source using Rust 1.60.0 or newer:

```bash
cargo install --git https://github.com/LPGhatguy/aftman
```

Once installed, you should add Aftman's bin directory to your system `PATH` environment variable.

- On Windows: `%USERPROFILE%\.aftman\bin`
- On Linux or macOS: `~/.aftman/bin`

## Getting Started
To create a new `aftman.toml` file in your current directory, run

```bash
aftman init
```

To add a new tool, you can follow the instructions in the file, or run

```bash
aftman add rojo-rbx/rojo

# install a specific version
aftman add rojo-rbx/rojo@6.2.0

# install with a different binary name
aftman add BurntSushi/ripgrep rg
```

If your PATH is configured correctly (see [Installation](#installation)), you will now be able to run that tool from your project.

To install a tool system-wide so that it can be used anywhere, edit `~/.aftman/aftman.toml` or run

```bash
aftman add --global rojo-rbx/rojo
```

## Supported Platforms
Aftman supports:

- Windows (x86, x86-64)
- macOS (x86-64, AArch64)
- Linux (x86, x86-64)

## Subcommands
For detailed help information, run `aftman --help`.

### `aftman init`
Usage:

```bash
aftman init [path]
```

Creates a new `aftman.toml` file in the given directory. Defaults to the current directory.

### `aftman add`
Usage:

```bash
aftman add [--global] <tool-spec> [tool-alias]
```

Installs a new tool with the given tool spec and optional alias to use for installing the tool.

Examples:

```bash
# Install the latest version of Rojo in the nearest aftman.toml file
aftman add rojo-rbx/rojo

# Install the latest version of Rojo globally
aftman add --global rojo-rbx/rojo

# Install a specific version of Rojo locally
aftman add rojo-rbx/rojo@6.2.0

# Install Rojo with a different binary name
aftman add rojo-rbx/rojo@6.2.0 rojo6
```

### `aftman self-install`
Usage:

```bash
aftman self-install
```

Installs Aftman inside itself and upgrades any references to Aftman.

Whenever you upgrade Aftman, run this command. Aftman makes copies of itself to mimic the tools it installs, and this command will ensure those copies get updated as well.

### `aftman list`
**This subcommand is not yet implemented.**

### `aftman update`
**This subcommand is not yet implemented.**

## Differences from Foreman
Aftman is spiritually very similar to [Foreman], a project I created at Roblox.

I'm hoping to fix some of the core design mistakes I made in Foreman and also take a little more care with the codebase. Roughly:

* **Exact version dependencies.** Using a range here has tripped up lots of users, so Aftman uses exact versions in all configuration files.
* **Commands to install, uninstall, and upgrade tools.** Editing a global, tucked-away toml file by hand is rough.
* **Change model to no longer trust-by-default.** Aftman prompts before downloading new tools. ([Roblox/foreman#16]).
* **Better strategy for storing executables.** ([Roblox/foreman#11])
* **Better heuristics for picking the right artifacts for your platform.** Aftman uses your Compiler, OS, architecture, and will eventually support custom patterns. ([Roblox/foreman#18])
* **Proper error handling.** Unlike Foreman, which uses `Result::unwrap` liberally, Aftman has good error hygeine with helpful context attached.
* **Less Roblox-angled.** Aftman does not market itself as being for Roblox development. It is a generally useful tool that can install all sorts of CLI tools.

[Foreman]: https://github.com/Roblox/foreman
[Roblox/foreman#11]: https://github.com/Roblox/foreman/issues/11
[Roblox/foreman#16]: https://github.com/Roblox/foreman/issues/16
[Roblox/foreman#18]: https://github.com/Roblox/foreman/issues/18