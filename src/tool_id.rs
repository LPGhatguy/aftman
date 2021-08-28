use std::fmt;
use std::str::FromStr;

use anyhow::{format_err, Context};
use semver::Version;
use serde::de::{Deserialize, Deserializer, Error, Visitor};
use serde::ser::{Serialize, Serializer};

use crate::ident::check_ident;

#[derive(Debug, PartialEq, Eq)]
pub struct ToolId {
    scope: String,
    name: String,
    version: Version,
}

impl ToolId {
    pub fn new<S, N>(scope: S, name: N, version: Version) -> anyhow::Result<Self>
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

    pub fn version(&self) -> &Version {
        &self.version
    }
}

impl FromStr for ToolId {
    type Err = anyhow::Error;

    fn from_str(value: &str) -> anyhow::Result<Self> {
        let context = || {
            format_err!(
                "Invalid Tool ID \"{}\". It must be of the form SCOPE/NAME@VERSION.",
                value
            )
        };

        let mut scope_rest = value.splitn(2, '/');
        let scope = scope_rest.next().unwrap();

        let mut name_version = scope_rest
            .next()
            .ok_or_else(|| format_err!("NAME is missing."))
            .with_context(context)?
            .splitn(2, '@');
        let name = name_version.next().unwrap();

        let version = name_version
            .next()
            .ok_or_else(|| format_err!("VERSION is missing."))
            .with_context(context)?
            .parse()
            .context("Invalid version")
            .with_context(context)?;

        Self::new(scope, name, version).with_context(context)
    }
}

impl Serialize for ToolId {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let formatted = format!("{}/{}@{}", self.scope, self.name, self.version);
        serializer.serialize_str(&formatted)
    }
}

impl<'de> Deserialize<'de> for ToolId {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        deserializer.deserialize_str(ToolSpecVisitor)
    }
}

struct ToolSpecVisitor;

impl<'de> Visitor<'de> for ToolSpecVisitor {
    type Value = ToolId;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "a Tool ID of the form SCOPE/NAME@VERSION")
    }

    fn visit_str<E: Error>(self, value: &str) -> Result<Self::Value, E> {
        value.parse().map_err(|err| E::custom(err))
    }
}

#[cfg(test)]
mod test {
    use super::*;

    /// Utility to create a ToolSpec for creating quick test cases.
    fn spec(scope: &str, name: &str, version: &str) -> ToolId {
        let version = Version::parse(version).expect("failed to create test Version");
        ToolId::new(scope, name, version).expect("failed to create test ToolId")
    }

    #[test]
    fn parse_success() {
        fn test(input: &str, expected: ToolId) {
            let parsed: ToolId = input.parse().expect("failed to parse ToolId");
            assert_eq!(parsed, expected);
        }

        test("a/b@1.0.0", spec("a", "b", Some("1.0.0")));
    }

    #[test]
    fn parse_failure() {
        fn test(input: &str, fragments: &[&str]) {
            let result: Result<ToolId, _> = input.parse();
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

        test("a/b", &["version is missing"]);
        test("hello/world", &["version is missing"]);

        test("abc/abc@", &["version must be non-empty"]);
        test("abc/abc@1", &["invalid version"]);
    }

    #[test]
    fn parse_json() {
        fn test(input: &str, expected: ToolId) {
            let parsed: ToolId = serde_json::from_str(input).expect("failed to parse ToolId");
            assert_eq!(parsed, expected);
        }

        test(r#""abc/abc@1.0.0""#, spec("abc", "abc", "1.0.0"));
    }
}
