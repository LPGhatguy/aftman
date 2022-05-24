## Changes Since Aftman $PREV_VERSION
$CHANGELOG

## Upgrading Aftman

### From GitHub Release
Download one of the attached binaries on this release page.

Once you have it, you can use it to install Aftman or upgrade your current install:

```bash
./aftman self-install
```

### From Crates.io
You can use Cargo (1.58.0+) to build this release yourself from crates.io:

```bash
cargo install aftman --version $VERSION
```

Afterwards, make sure to run `aftman self-install` to upgrade all links to the new version of Aftman.