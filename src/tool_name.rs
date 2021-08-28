use std::fmt;
use std::str::FromStr;

use anyhow::{format_err, Context};
use serde::de::{Deserialize, Deserializer, Error, Visitor};
use serde::ser::{Serialize, Serializer};

use crate::ident::check_ident;

#[derive(Debug, PartialEq, Eq)]
pub struct ToolName {
    scope: String,
    name: String,
}

impl ToolName {
    pub fn new<S, N>(scope: S, name: N) -> anyhow::Result<Self>
    where
        S: Into<String>,
        N: Into<String>,
    {
        let scope = scope.into();
        let name = name.into();

        check_ident("Scope", &scope)?;
        check_ident("Name", &name)?;

        Ok(Self { scope, name })
    }

    pub fn scope(&self) -> &str {
        &self.scope
    }

    pub fn name(&self) -> &str {
        &self.name
    }
}

impl fmt::Display for ToolName {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "{}/{}", self.scope, self.scope)
    }
}

impl FromStr for ToolName {
    type Err = anyhow::Error;

    fn from_str(value: &str) -> anyhow::Result<Self> {
        let context = || {
            format_err!(
                "Invalid Tool Name \"{}\". It must be of the form SCOPE/NAME.",
                value
            )
        };

        let mut scope_rest = value.splitn(2, '/');
        let scope = scope_rest.next().unwrap();

        let name = scope_rest
            .next()
            .ok_or_else(|| format_err!("NAME is missing."))
            .with_context(context)?;

        Self::new(scope, name).with_context(context)
    }
}

impl Serialize for ToolName {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(&self.to_string())
    }
}

impl<'de> Deserialize<'de> for ToolName {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        deserializer.deserialize_str(ToolNameVisitor)
    }
}

struct ToolNameVisitor;

impl<'de> Visitor<'de> for ToolNameVisitor {
    type Value = ToolName;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "a Tool Name of the form SCOPE/NAME")
    }

    fn visit_str<E: Error>(self, value: &str) -> Result<Self::Value, E> {
        value.parse().map_err(|err| E::custom(err))
    }
}

#[cfg(test)]
mod test {
    use super::*;

    /// Utility to create a ToolName for creating quick test cases.
    fn name(scope: &str, name: &str) -> ToolName {
        ToolName::new(scope, name).expect("failed to create test ToolName")
    }

    #[test]
    fn parse_success() {
        fn test(input: &str, expected: ToolName) {
            let parsed: ToolName = input.parse().expect("failed to parse ToolName");
            assert_eq!(parsed, expected);
        }

        test("a/b", name("a", "b"));
        test("hello/world", name("hello", "world"));
    }

    #[test]
    fn parse_failure() {
        fn test(input: &str, fragments: &[&str]) {
            let result: Result<ToolName, _> = input.parse();
            let err = format!("{:?}", result.expect_err("succeeded parsing bad ToolName"));
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
    }

    #[test]
    fn parse_json() {
        fn test(input: &str, expected: ToolName) {
            let parsed: ToolName = serde_json::from_str(input).expect("failed to parse ToolName");
            assert_eq!(parsed, expected);
        }

        test(r#""abc/def""#, name("abc", "def"));
    }
}
