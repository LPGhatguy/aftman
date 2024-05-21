# Aftman Changelog

## Unreleased Changes

## [0.3.0] (May 11, 2024)
* Switched from OpenSSL to rustls, which should fix some dynamic dependency issues on certain distributions. ([#62])
* Added support for Linux AArch64 builds ([#64])

[#62]: https://github.com/LPGhatguy/aftman/pull/62
[#64]: https://github.com/LPGhatguy/aftman/pull/64
[0.3.0]: https://github.com/LPGhatguy/aftman/releases/tag/v0.3.0

## [0.2.8] (May 2, 2024)
* Improved trust check logic and added `--skip-untrusted` flag ([#38])
* Fixed macOS AArch64 builds actually being x86-64 ([#34])
* Fixed environment configuration on zsh under some scenarios ([#45])
* Fixed handling of uppercase `.EXE` suffix binaries on Windows ([#57])
* Minor version bumps to various dependencies

[#34]: https://github.com/LPGhatguy/aftman/pull/34
[#38]: https://github.com/LPGhatguy/aftman/pull/38
[#45]: https://github.com/LPGhatguy/aftman/pull/45
[#57]: https://github.com/LPGhatguy/aftman/pull/57
[0.2.8]: https://github.com/LPGhatguy/aftman/releases/tag/v0.2.8

## [0.2.7] (September 6, 2022)
* Added support for authenticating with GitHub. ([#18])
	* After updating Aftman, edit `~/.aftman/auth.toml` to add a [Personal Access Token][pat].
* Migrated from structopt to clap. ([#29])

[#18]: https://github.com/LPGhatguy/aftman/pull/18
[#29]: https://github.com/LPGhatguy/aftman/pull/29
[pat]: https://docs.github.com/en/authentication/keeping-your-account-and-data-secure/creating-a-personal-access-token
[0.2.7]: https://github.com/LPGhatguy/aftman/releases/tag/v0.2.7

## [0.2.6] (August 11, 2022)
* Added `aftman list` subcommand. ([#27])
* Added support for installing macOS artifacts with the "darwin" keyword. ([#21])
* Fixed tool exit codes not being propagated. ([#25])
* Fixed `self-install` not creating a `.zshenv` file on macOS. ([#28])

[#21]: https://github.com/LPGhatguy/aftman/pull/21
[#25]: https://github.com/LPGhatguy/aftman/pull/25
[#27]: https://github.com/LPGhatguy/aftman/pull/27
[#28]: https://github.com/LPGhatguy/aftman/pull/28
[0.2.6]: https://github.com/LPGhatguy/aftman/releases/tag/v0.2.6

## [0.2.5] (July 2, 2022)
* Improved `self-install` behavior. ([#20])

[#20]: https://github.com/LPGhatguy/aftman/pull/20
[0.2.5]: https://github.com/LPGhatguy/aftman/releases/tag/v0.2.5

## [0.2.4] (July 1, 2022)
* Added support for `aftman self-install` on Unix platform. ([#16])
* Fixed Linux releases running on systems with an older glibc.

[#16]: https://github.com/LPGhatguy/aftman/pull/16
[0.2.4]: https://github.com/LPGhatguy/aftman/releases/tag/v0.2.4

## [0.2.3] (May 24, 2022)
* Fixed subprocesses living forever on Unix platforms. ([#13])
* Aftman now correctly marks executables as executable on Unix platforms. ([#14])

[#13]: https://github.com/LPGhatguy/aftman/pull/13
[#14]: https://github.com/LPGhatguy/aftman/pull/14
[0.2.3]: https://github.com/LPGhatguy/aftman/releases/tag/v0.2.3

## [0.2.2] (May 23, 2022)
* Fixed building on non-Windows platforms
* Started publishing binaries for several platforms:
	* Windows (x86_64)
	* Linux (x86_64)
	* macOS (x86_64 and AArch64)

[0.2.2]: https://github.com/LPGhatguy/aftman/releases/tag/v0.2.2

## 0.2.1 (May 23, 2022)
* Added `aftman install`
* Added `aftman trust`

## 0.2.0 (May 23, 2022)
* Initial release