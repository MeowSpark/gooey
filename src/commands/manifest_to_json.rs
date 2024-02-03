use std::path::PathBuf;

use structopt::StructOpt;

use crate::package_compat;

/// Print a gooey manifest as a line of JSON.
///
/// Used for creating the gooey package index.
#[derive(Debug, StructOpt)]
pub struct ManifestToJsonSubcommand {
    /// Path to the project to output the manifest of.
    #[structopt(long = "project-path", default_value = ".")]
    pub project_path: PathBuf,
}

impl ManifestToJsonSubcommand {
    pub fn run(self) -> anyhow::Result<()> {
        let manifest = package_compat::load_backwards_compatible_package(&self.project_path)?;
        println!("{}", serde_json::to_string(&manifest)?);

        Ok(())
    }
}
