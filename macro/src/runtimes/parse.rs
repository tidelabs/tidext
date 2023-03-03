use darling::FromMeta;
use quote::format_ident;

#[derive(FromMeta)]
pub struct RuntimeMetadataPath(String);

#[derive(FromMeta)]
pub struct RuntimeMetadataArgs {
  pub runtime_metadata_path: RuntimeMetadataPath,
}

#[derive(Clone, Debug)]
pub struct Runtime {
  pub spec_name: String,
  pub spec_version: u32,
  pub enum_name: syn::Ident,
  pub mod_name: syn::Ident,
  pub runtime_name: syn::Ident,
  pub runtime_metadata_path: String,
}

pub trait SortRuntime {
  fn sort_desc(&self) -> Self;
}

impl SortRuntime for Vec<Runtime> {
  fn sort_desc(&self) -> Self {
    let mut runtimes = self.clone();
    runtimes.sort_by(|a, b| {
      b.spec_version
        .partial_cmp(&a.spec_version)
        .unwrap_or(std::cmp::Ordering::Equal)
    });
    runtimes.to_vec()
  }
}

impl TryInto<Vec<Runtime>> for RuntimeMetadataPath {
  type Error = std::io::Error;
  fn try_into(self) -> Result<Vec<Runtime>, Self::Error> {
    let mut all_runtimes = Vec::new();
    let root = std::env::var("CARGO_MANIFEST_DIR").unwrap_or_else(|_| ".".into());
    let root_path = std::path::Path::new(&root);

    let paths = std::fs::read_dir(root_path.join(&self.0))?;

    for path in paths {
      let path = path?;
      let dir_path = root_path.join(path.path());

      let spec_version = path
        .path()
        .components()
        .last()
        .ok_or(std::io::Error::new(
          std::io::ErrorKind::InvalidInput,
          "Invalid artifacts directory",
        ))?
        .as_os_str()
        .to_str()
        .ok_or(std::io::Error::new(
          std::io::ErrorKind::InvalidInput,
          "Invalid artifacts directory",
        ))?
        .parse::<u32>()
        .map_err(|_| {
          std::io::Error::new(
            std::io::ErrorKind::InvalidInput,
            "Invalid artifacts directory",
          )
        })?;

      // lagoon
      let expected_lagoon_path = dir_path.join("lagoon_metadata.scale");
      if expected_lagoon_path.exists() {
        all_runtimes.push(Runtime {
          spec_name: "lagoon".into(),
          enum_name: format_ident!("Lagoon{}Runtime", spec_version),
          spec_version,
          runtime_metadata_path: expected_lagoon_path
            .to_str()
            .ok_or(std::io::Error::new(
              std::io::ErrorKind::InvalidInput,
              "Invalid artifacts directory",
            ))?
            .to_string(),
          mod_name: format_ident!("lagoon{}", spec_version),
          runtime_name: format_ident!("{}", "lagoon_runtime"),
        })
      }

      // tidechain
      let expected_tidechain_path = dir_path.join("tidechain_metadata.scale");
      if expected_tidechain_path.exists() {
        all_runtimes.push(Runtime {
          spec_name: "tidechain".into(),
          enum_name: format_ident!("Tidechain{}Runtime", spec_version),
          spec_version,
          runtime_metadata_path: expected_tidechain_path
            .to_str()
            .ok_or(std::io::Error::new(
              std::io::ErrorKind::InvalidInput,
              "Unable to convert path to string",
            ))?
            .to_string(),
          mod_name: format_ident!("tidechain{}", spec_version),
          runtime_name: format_ident!("{}", "tidechain_runtime"),
        })
      }
    }

    if all_runtimes.is_empty() {
      return Err(std::io::Error::new(
        std::io::ErrorKind::InvalidInput,
        "Invalid artifacts directory, no metadata found",
      ));
    }

    Ok(all_runtimes)
  }
}
