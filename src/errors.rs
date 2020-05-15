use failure_derive::Fail;

/// Error types for kvs
#[derive(Debug, Fail)]
pub enum KvsError {
    /// Unknown error 
    #[fail(display = "Unknown error occurred.")]
    Unknown,    
}

/// Result type for KvStore.
pub type Result<T> = std::result::Result<T, KvsError>;
