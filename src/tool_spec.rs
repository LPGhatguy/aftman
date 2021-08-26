use std::fmt;
use std::str::FromStr;

use semver::Version;
use serde::de::{Deserialize, Deserializer, Error, Visitor};
use serde::ser::{Serialize, Serializer};

use anyhow::{ensure, format_err, Context};

#[derive(Debug, PartialEq, Eq)]
pub struct ToolSpec {
    scope: String,
    name: String,
    version: Option<Version>,
}

impl ToolSpec {
    pub fn new<S, N>(scope: S, name: N, version: Option<Version>) -> anyhow::Result<Self>
    where
        S: Into<String>,
        N: Into<String>,
    {
        let scope = scope.into();
        let name = name.into();

        scope.chars().any(char::is_whitespace);

        check_ident("Scope", &scope)?;
        check_ident("Name", &name)?;

        Ok(Self {
            scope,
            name,
            version,
        })
    }

    pub fn scope(&self) -> &str {
        &self.scope
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn version(&self) -> Option<&Version> {
        self.version.as_ref()
    }
}

fn check_ident(ident_type: &str, ident: &str) -> anyhow::Result<()> {
    ensure!(ident.len() > 0, "{} must be non-empty", ident_type);
    ensure!(
        !ident.chars().all(char::is_whitespace),
        "{} must be non-empty",
        ident_type
    );
    ensure!(
        ident.chars().all(|c| c != '/'),
        "{} must not contain a slash"
    );

    Ok(())
}

impl FromStr for ToolSpec {
    type Err = anyhow::Error;

    fn from_str(value: &str) -> anyhow::Result<Self> {
        let context = || {
            format_err!("Invalid Tool Spec \"{}\". It must be of the form SCOPE/NAME or SCOPE/NAME@VERSION.", value)
        };

        let mut scope_rest = value.splitn(2, '/');
        let scope = scope_rest.next().unwrap();

        let mut name_version = scope_rest
            .next()
            .ok_or_else(|| format_err!("NAME is missing."))
            .with_context(context)?
            .splitn(2, '@');
        let name = name_version.next().unwrap();

        let version = match name_version.next() {
            None => None,
            Some(version_str) => {
                if version_str.len() == 0 || version_str.chars().all(char::is_whitespace) {
                    return Err(format_err!("VERSION must be non-empty.")).with_context(context);
                }

                let version = version_str
                    .parse()
                    .context("Invalid version")
                    .with_context(context)?;
                Some(version)
            }
        };

        Self::new(scope, name, version).with_context(context)
    }
}

impl Serialize for ToolSpec {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let formatted = match &self.version {
            Some(version) => format!("{}/{}@{}", self.scope, self.name, version),
            None => format!("{}/{}", self.scope, self.name),
        };
        serializer.serialize_str(&formatted)
    }
}

impl<'de> Deserialize<'de> for ToolSpec {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        deserializer.deserialize_str(ToolSpecVisitor)
    }
}

struct ToolSpecVisitor;

impl<'de> Visitor<'de> for ToolSpecVisitor {
    type Value = ToolSpec;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(
            formatter,
            "a Tool Spec of the form SCOPE/NAME or SCOPE/NAME@VERSION"
        )
    }

    fn visit_str<E: Error>(self, value: &str) -> Result<Self::Value, E> {
        value.parse().map_err(|err| E::custom(err))
    }
}

#[cfg(test)]
mod test {
    use super::*;

    /// Utility to create a ToolSpec for creating quick test cases.
    fn spec(scope: &str, name: &str, version: Option<&str>) -> ToolSpec {
        let version = match version {
            Some(v) => Some(Version::parse(v).expect("failed to create test Version")),
            None => None,
        };
        ToolSpec::new(scope, name, version).expect("failed to create test ToolSpec")
    }

    #[test]
    fn parse_success() {
        fn test(input: &str, expected: ToolSpec) {
            let parsed: ToolSpec = input.parse().expect("failed to parse ToolSpec");
            assert_eq!(parsed, expected);
        }

        test("a/b", spec("a", "b", None));
        test("hello/world", spec("hello", "world", None));
        test("a/b@1.0.0", spec("a", "b", Some("1.0.0")));
    }

    #[test]
    fn parse_failure() {
        fn test(input: &str, fragments: &[&str]) {
            let result: Result<ToolSpec, _> = input.parse();
            let err = format!("{:?}", result.expect_err("succeeded parsing bad ToolSpec"));
            let err_lowercase = err.to_lowercase();

            if fragments.is_empty() {
                panic!(
                    "Debug output, no fragments specified. Error message:\n{}",
                    err
                );
            }

            for fragment in fragments {
                if !err_lowercase.contains(fragment) {
                    panic!(
                        "Expected error to contain '{}' but it did not. Error:\n{}",
                        fragment, err
                    );
                }
            }
        }

        test("", &["name is missing"]);
        test("abc", &["name is missing", "abc"]);

        test("/", &["scope must be non-empty"]);
        test("/abc", &["scope must be non-empty"]);

        test("abc/", &["name must be non-empty"]);
        test("abc/ ", &["name must be non-empty"]);

        test("abc/abc@", &["version must be non-empty"]);
        test("abc/abc@1", &["invalid version"]);
    }

    #[test]
    fn parse_json() {
        fn test(input: &str, expected: ToolSpec) {
            let parsed: ToolSpec = serde_json::from_str(input).expect("failed to parse ToolSpec");
            assert_eq!(parsed, expected);
        }

        test(r#""abc/abc@1.0.0""#, spec("abc", "abc", Some("1.0.0")));
    }
}
