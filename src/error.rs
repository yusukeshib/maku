use thiserror::Error;

#[derive(Error, Debug)]
pub enum MakuError {
    #[error("Image error")]
    Image(#[from] image::ImageError),
    #[error("Project loading error")]
    ProjectLoad(#[from] serde_json::Error),
}
