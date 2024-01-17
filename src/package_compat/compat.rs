use std::ffi::OsString;
use std::fs::DirEntry;
use std::path::PathBuf;
use crate::manifest::MANIFEST_FILE_NAME;
use crate::package_compat::WALLY_MANIFEST_FILE_NAME;

#[derive(Debug)]
pub struct PackageCompatibility {
    /// The path of the package to open.
    path: PathBuf,
}

#[derive(Debug)]
struct PackageCompatibilityLayer {
    /// The packages manifest name
    manifest_name: *const str,

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

fn handle_based_on_file_path(path: PathBuf) -> anyhow::Result<Option<String>> {
    let mut best_manifest_name : Option<String> = None;
    let mut best_layer_entry : Option<DirEntry> = None;
    let mut best_layer_weight = 99;

    for entry in path.read_dir()? {
        let pos_best_layer : DirEntry = entry.unwrap();

        let name : OsString = pos_best_layer.file_name();
        PACKAGE_COMPATIBILITY_LAYERS.iter().find(|layer| {
            layer.manifest_name == name.as_os_str().to_str().unwrap_or("") && layer.weight < best_layer_weight
        }).map(|layer| {
            best_manifest_name = Some(layer.manifest_name.parse().unwrap());
            best_layer_entry = Some(pos_best_layer);
            best_layer_weight = layer.weight;
        });
    }
    Ok(best_manifest_name)
}

impl PackageCompatibility {
    pub fn load_backwards_compatible_package(self) -> anyhow::Result<()> {
        let best_layer_name = handle_based_on_file_path(self.path).expect("Fatal error while choosing package compatibility layer");

        match best_layer_name {
            None => anyhow::bail!("No available package compatibility layer found"),
            Some(layer) => {
                match layer.as_str() { // TODO
                    MANIFEST_FILE_NAME => {}
                    WALLY_MANIFEST_FILE_NAME => {}
                    _ => anyhow::bail!("Unknown package compatibility layer {}", layer),
                }
            }
        }
        anyhow::bail!("No available package compatibility layer found")
    }
}