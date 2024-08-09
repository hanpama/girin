#[derive(Debug)]
pub enum PythonRenderingError {
    Io(std::io::Error),
}

impl From<std::io::Error> for PythonRenderingError {
    fn from(e: std::io::Error) -> Self {
        PythonRenderingError::Io(e)
    }
}
