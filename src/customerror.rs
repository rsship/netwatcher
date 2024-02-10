#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Failed to execute `{0}`. Received error code `{1}`")]
    GetIfAddrsError(String, i32),
    #[error("Failed to execute `{0}`. Received error code `{1}`")]
    GetIfNameError(String, u32),
}
