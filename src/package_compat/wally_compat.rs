use crate::manifest::{Manifest, Realm};
use crate::package_compat::wally_manifest::{WallyManifest, WallyRealm};
use crate::package_name::PackageName;


pub fn load_as_backwards_compatible_package(manifest: WallyManifest) -> anyhow::Result<Manifest> {
    let mut new_manifest = Manifest::new()?;

    new_manifest.package.name = PackageName::new(manifest.package.name.scope(), manifest.package.name.name())?;
    match manifest.package.realm {
        WallyRealm::Server => {new_manifest.package.realm = Realm::Server}
        WallyRealm::Shared => {new_manifest.package.realm = Realm::Shared}
        WallyRealm::Dev => {new_manifest.package.realm = Realm::Dev}
    }



    // TODO: This will obviously not be a 1:1 clone in the near future
    new_manifest.package.version = manifest.package.version.clone();
    new_manifest.package.registry = manifest.package.registry.clone();
    new_manifest.package.description = manifest.package.description.clone();
    new_manifest.package.license = manifest.package.license.clone();
    new_manifest.package.authors = manifest.package.authors.clone();
    new_manifest.package.include = manifest.package.include.clone();
    new_manifest.package.exclude = manifest.package.exclude.clone();
    new_manifest.package.private = manifest.package.private.clone();

    Ok(new_manifest)
}