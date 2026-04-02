pub mod camera;
pub mod lisp;
pub mod plane;
pub mod scene;
pub mod sphere;
mod texture;
pub mod types;
mod vec;

#[derive(Debug, Clone)]
pub enum RTError {
    EvalError(lispers_core::lisp::eval::EvalError),
    FFMpegError(video_rs::Error),
}

impl From<lispers_core::lisp::eval::EvalError> for RTError {
    fn from(value: lispers_core::lisp::eval::EvalError) -> Self {
        RTError::EvalError(value)
    }
}

impl From<video_rs::Error> for RTError {
    fn from(value: video_rs::Error) -> Self {
        RTError::FFMpegError(value)
    }
}
