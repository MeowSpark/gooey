use std::ffi::OsString;
use std::fs::DirEntry;
use std::path::{Path};
use crate::manifest::{Manifest, MANIFEST_FILE_NAME};
use crate::package_compat::{wally_compat, WALLY_MANIFEST_FILE_NAME};
use crate::package_compat::wally_manifest::WallyManifest;

/// package cross compatibility layer management

#[derive(Debug)]
struct PackageCompatibilityLayer<'a> {
    /// The packages manifest name
    manifest_name: &'a str,

    /// Weight of layer
    weight: u8,
}

const PACKAGE_COMPATIBILITY_LAYERS: [PackageCompatibilityLayer; 3] = [
    PackageCompatibilityLayer {
        manifest_name: MANIFEST_FILE_NAME,
        weight: 0,
    },
    PackageCompatibilityLayer {
        manifest_name: "rotriever.toml",
        weight: 1,
    },
    PackageCompatibilityLayer {
        manifest_name: WALLY_MANIFEST_FILE_NAME,
        weight: 2,
    },
];

fn handle_based_on_file_path(path: &Path) -> anyhow::Result<Option<String>> {
    let mut best_manifest_name : Option<String> = None;
    let mut best_layer_entry : Option<DirEntry> = None;
    let mut best_layer_weight = 99;

    for entry in path.read_dir()? {
        let pos_best_layer : DirEntry = entry.unwrap();

        let name : OsString = pos_best_layer.file_name();
        PACKAGE_COMPATIBILITY_LAYERS.iter().find(|layer| {
            layer.manifest_name == name.as_os_str().to_str().unwrap_or("") && layer.weight < best_layer_weight
        }).map(|layer| {
            best_manifest_name = Some(layer.manifest_name.to_string());
            best_layer_entry = Some(pos_best_layer);
            best_layer_weight = layer.weight;
        });
    }
    Ok(best_manifest_name)
}


pub fn load_backwards_compatible_package(path: &Path) -> anyhow::Result<Manifest> {
    let best_layer_name = handle_based_on_file_path(path)?;

    match best_layer_name {
        Some(layer) => {
            match layer.as_str() {
                MANIFEST_FILE_NAME => {
                    let manifest = Manifest::load(&path)?;
                    log::debug!("Using normal manifest");
                    Ok(manifest)
                }, // load manifest normally, return normal manifest.
                WALLY_MANIFEST_FILE_NAME => {
                    let wally_manifest = WallyManifest::load(&path)?;
                    let manifest = wally_compat::load_as_backwards_compatible_package(wally_manifest)?;
                    log::debug!("Using Wally manifest");
                    Ok(manifest)
                }, // load wally manifest & pass to wally_compat, then return normal manifest.
                _ => anyhow::bail!("Unknown package compatibility layer {}", layer),
            }
        },
        None => anyhow::bail!("No available package compatibility layer found")
    }
}