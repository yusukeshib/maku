pub mod composition;
pub mod error;
pub mod io;
pub mod target;

use error::MakuError;
use io::IoComposition;

/// Main structure for the Maku image processing system
pub struct Maku {
    root: composition::Composition,
}

impl Maku {
    /// Load a Maku instance from a JSON configuration file
    pub async fn load(
        context: &three_d::Context,
        json_path: std::path::PathBuf,
    ) -> Result<Maku, MakuError> {
        log::debug!("Load json: {:?}", json_path);
        let json = std::fs::read_to_string(json_path.clone())?;
        let composition = serde_json::from_str::<IoComposition>(&json).map_err(MakuError::from)?;
        let parent_dir = json_path
            .parent()
            .unwrap_or_else(|| std::path::Path::new("."));
        let root = composition::Composition::load(context, &composition, parent_dir).await?;
        Ok(Self { root })
    }

    pub fn render(&mut self, target: &mut target::Target) -> Result<(), MakuError> {
        self.root.render(target)
    }

    pub fn render_to_file(
        &mut self,
        context: &three_d::Context,
        output_path: std::path::PathBuf,
    ) -> Result<(), MakuError> {
        self.root.render_to_file(context, output_path)
    }
}
