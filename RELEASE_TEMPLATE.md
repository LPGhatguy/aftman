## Changes Since $PROJECT_NAME $PREV_VERSION
$CHANGELOG

## Upgrading $PROJECT_NAME

### Self-Upgrading With Aftman
Aftman can upgrade itself by using `aftman add`:

```bash
aftman add --global LPGhatguy/aftman
```

### From GitHub Release
Download one of the attached binaries on this release page!

### From Crates.io
You can use Cargo (1.58.0+) to build this release yourself from crates.io:

```bash
cargo install aftman --version $VERSION
```