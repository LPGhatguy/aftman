# Aftman
Aftman is a work-in-progess toolchain manager. It is the spiritual successor to [Foreman], a toolchain manager I created at Roblox.

## Supported Platforms
Because Aftman has to make decisions about what release artifacts might be compatible with the host platform, it has relatively limited platform support:

- Windows (x86, x86-64)
- macOS (x86-64, AArch64)
- Linux (x86, x86-64)

## Differences from Foreman
I'm hoping to fix some of the core design mistakes I made in Foreman and also take a little more care with the codebase. Roughly:

* **Exact version dependencies.** Using a range here has tripped up lots of users and I realized at some point that this isn't great.
* **Commands to install, uninstall, and upgrade tools.** Editing a global, tucked-away toml file by hand is rough.
* **Change model to no longer trust-by-default.** Aftman will prompt before downloading new tools. ([Roblox/foreman#16]).
* **Better strategy for storing executables.** ([Roblox/foreman#11])
* **Better heuristics for picking the right artifacts for your platform.** Compiler, OS, architecture, plus custom patterns. ([Roblox/foreman#18])
* **Proper error handling.** I used `unwrap` all over because Foreman was meant to be a prototype. This should make diagnosing issues a lot easier.
* **Less Roblox-angled.** I think that this sort of tool would be useful in general, not just for the Roblox community, so I want to avoid making it overtly Roblox.

[Foreman]: https://github.com/Roblox/foreman
[Roblox/foreman#11]: https://github.com/Roblox/foreman/issues/11
[Roblox/foreman#16]: https://github.com/Roblox/foreman/issues/16
[Roblox/foreman#18]: https://github.com/Roblox/foreman/issues/18