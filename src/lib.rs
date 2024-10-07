pub mod error;
pub mod io;

use error::MakuError;
use io::IoProject;

pub struct Filter {}

pub struct Maku {
    filters: Vec<Filter>,
}

impl Maku {
    pub fn new() -> Self {
        Self { filters: vec![] }
    }

    pub fn load(json: &str) -> Result<Maku, MakuError> {
        let _project = serde_json::from_str::<IoProject>(json).map_err(MakuError::from)?;
        let maku = Self::new();
        // TODO: filters
        Ok(maku)
    }

    pub fn add_filter(&mut self, filter: Filter) {
        todo!()
    }
}
