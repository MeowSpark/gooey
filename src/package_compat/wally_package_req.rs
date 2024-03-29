use std::fmt;
use std::str::FromStr;

use anyhow::{anyhow, bail, Context};
use semver::{Version, VersionReq};
use serde::de::{Deserialize, Deserializer, Error, Visitor};
use serde::ser::{Serialize, Serializer};

use crate::package_compat::wally_package_id::WallyPackageId;
use crate::package_compat::wally_package_name::WallyPackageName;

/// Describes a requirement on a package, consisting of a scope, name, and valid
/// version range.
///
/// Examples of package requirements:
/// * `roblox/roact@1.4.2`
/// * `lpghatguy/asink@0.2.0-alpha.3`
/// * `foo/bar@1`
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct WallyPackageReq {
    name: WallyPackageName,
    version_req: VersionReq,
}

impl WallyPackageReq {
    pub fn new(name: WallyPackageName, version_req: VersionReq) -> Self {
        WallyPackageReq { name, version_req }
    }

    pub fn name(&self) -> &WallyPackageName {
        &self.name
    }

    pub fn version_req(&self) -> &VersionReq {
        &self.version_req
    }

    pub fn matches_id(&self, package_id: &WallyPackageId) -> bool {
        self.matches(package_id.name(), package_id.version())
    }

    pub fn matches(&self, name: &WallyPackageName, version: &Version) -> bool {
        self.name() == name && self.version_req.matches(version)
    }
}

impl fmt::Display for WallyPackageReq {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "{}@{}", self.name, self.version_req)
    }
}

impl FromStr for WallyPackageReq {
    type Err = anyhow::Error;

    fn from_str(value: &str) -> anyhow::Result<Self> {
        const BAD_FORMAT_MSG: &str = "a package requirement is of the form SCOPE/NAME@VERSION_REQ";

        let mut first_half = value.splitn(2, '/');
        let scope = first_half.next().ok_or_else(|| anyhow!(BAD_FORMAT_MSG))?;
        let name_and_version = first_half.next().ok_or_else(|| anyhow!(BAD_FORMAT_MSG))?;

        let mut second_half = name_and_version.splitn(2, '@');
        let name = second_half.next().ok_or_else(|| anyhow!(BAD_FORMAT_MSG))?;

        let version_req_source = second_half.next().ok_or_else(|| anyhow!(BAD_FORMAT_MSG))?;

        // The VersionReq type will successfully parse from an empty or
        // all-spaces string, yielding a wildcard. This is not behavior we want,
        // so let's check for that here.
        //
        // https://github.com/steveklabnik/semver-parser/issues/51
        if version_req_source.len() == 0 || version_req_source.chars().all(char::is_whitespace) {
            bail!(BAD_FORMAT_MSG);
        }

        let version_req = version_req_source
            .parse()
            .context("could not parse version requirement")?;

        let package_name = WallyPackageName::new(scope, name).context(BAD_FORMAT_MSG)?;
        Ok(WallyPackageReq::new(package_name, version_req))
    }
}

impl Serialize for WallyPackageReq {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let combined_name = format!(
            "{}/{}@{}",
            self.name().scope(),
            self.name().name(),
            self.version_req()
        );
        serializer.serialize_str(&combined_name)
    }
}

impl<'de> Deserialize<'de> for WallyPackageReq {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        deserializer.deserialize_str(PackageReqVisitor)
    }
}

struct PackageReqVisitor;

impl<'de> Visitor<'de> for PackageReqVisitor {
    type Value = WallyPackageReq;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(
            formatter,
            "a package requirement of the form SCOPE/NAME@VERSION_REQ"
        )
    }

    fn visit_str<E: Error>(self, value: &str) -> Result<Self::Value, E> {
        value.parse().map_err(|err| E::custom(err))
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn new() {
        let req = WallyPackageReq::new(
            WallyPackageName::new("foo", "bar").unwrap(),
            VersionReq::parse("1.2.3").unwrap(),
        );
        assert_eq!(req.name().scope(), "foo");
        assert_eq!(req.name().name(), "bar");
        assert_eq!(req.version_req(), &VersionReq::parse("1.2.3").unwrap());
    }

    #[test]
    fn display() {
        let req = WallyPackageReq::new(
            WallyPackageName::new("hello", "world").unwrap(),
            VersionReq::parse("0.2.3").unwrap(),
        );

        // The semver crate's VersionReq type stores and prints using the most
        // explicit version of a constraint. This normalization helps make
        // comparison and evaluation simpler, but make printing a little ugly.
        assert_eq!(req.to_string(), "hello/world@>=0.2.3, <0.3.0");
    }

    #[test]
    fn parse() {
        // If given a semver version, we default to the ^ operator, which means
        // "compatible with". This is a good default that Cargo also chooses.
        let default_compat: WallyPackageReq = "hello/world@1.2.3".parse().unwrap();
        assert_eq!(default_compat.name().scope(), "hello");
        assert_eq!(default_compat.name().name(), "world");
        assert_eq!(
            default_compat.version_req(),
            &VersionReq::parse("^1.2.3").unwrap()
        );

        // Arbitrarily complex semver predicates can be chained together. This
        // range might mean "0.2.7 is really broken and I don't want it".
        let with_ops: WallyPackageReq = "hello/world@>=0.2.0, <0.2.7".parse().unwrap();
        assert_eq!(with_ops.name().scope(), "hello");
        assert_eq!(with_ops.name().name(), "world");
        assert_eq!(
            with_ops.version_req(),
            &VersionReq::parse(">=0.2.0, <0.2.7").unwrap()
        );
    }

    #[test]
    fn parse_invalid() {
        // Package requirements require a version requirement.
        let no_version: Result<WallyPackageReq, _> = "hello/world".parse();
        no_version.unwrap_err();
        let no_version_at: Result<WallyPackageReq, _> = "hello/world@".parse();
        no_version_at.unwrap_err();
    }

    #[test]
    fn serialization() {
        let name = WallyPackageName::new("lpghatguy", "asink").unwrap();
        let package_req = WallyPackageReq::new(name, VersionReq::parse("2.3.1").unwrap());

        let serialized = serde_json::to_string(&package_req).unwrap();
        assert_eq!(serialized, "\"lpghatguy/asink@>=2.3.1, <3.0.0\"");

        let deserialized: WallyPackageReq = serde_json::from_str(&serialized).unwrap();
        assert_eq!(deserialized, package_req);
    }
}
