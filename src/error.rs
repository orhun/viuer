pub type ViuResult = std::result::Result<(), ViuError>;

#[derive(Debug)]
pub enum ViuError {
    Image(image::ImageError),
    IO(std::io::Error),
}

impl std::error::Error for ViuError {}

impl From<std::io::Error> for ViuError {
    fn from(err: std::io::Error) -> Self {
        ViuError::IO(err)
    }
}
impl From<image::ImageError> for ViuError {
    fn from(err: image::ImageError) -> Self {
        ViuError::Image(err)
    }
}

impl std::fmt::Display for ViuError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ViuError::Image(e) => write!(f, "Image error: {}", e),
            ViuError::IO(e) => write!(f, "IO error: {}", e),
        }
    }
}
