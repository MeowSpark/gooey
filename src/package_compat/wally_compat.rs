use crate::manifest::{Manifest, Realm};
use crate::package_compat::wally_manifest::{WallyManifest, WallyRealm};
use crate::package_compat::WallyPackage;
use crate::package_id::PackageId;
use crate::package_name::PackageName;

#[derive(Debug)]
pub struct WallyPackageCompatibility {
    /// The path of the package to open.
    manifest: WallyManifest,
}

impl WallyPackageCompatibility {
    pub fn new(manifest: WallyManifest) -> Self {
        Self {
            manifest,
        }
    }

    pub fn load_as_backwards_compatible_package(&self) -> anyhow::Result<Manifest> {
        let mut manifest = Manifest::new()?;

        manifest.package.name = PackageName::new(self.manifest.package.name.scope(), self.manifest.package.name.name())?;
        match self.manifest.package.realm {
            WallyRealm::Server => {manifest.package.realm = Realm::Server}
            WallyRealm::Shared => {manifest.package.realm = Realm::Shared}
            WallyRealm::Dev => {manifest.package.realm = Realm::Dev}
        }
            
        

        // TODO: This will obviously not be a 1:1 clone in the near future
        manifest.package.version = self.manifest.package.version.clone();
        manifest.package.registry = self.manifest.package.registry.clone();
        manifest.package.description = self.manifest.package.description.clone();
        manifest.package.license = self.manifest.package.license.clone();
        manifest.package.authors = self.manifest.package.authors.clone();
        manifest.package.include = self.manifest.package.include.clone();
        manifest.package.exclude = self.manifest.package.exclude.clone();
        manifest.package.private = self.manifest.package.private.clone();

        Ok(manifest)
    }
}