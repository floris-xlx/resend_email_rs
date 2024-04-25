//! ## Error Module
//! 
//! This module defines the various error types that can occur within the Resend email service.
//! 
//! ## Table of Contents
//! - [Error](#error)
//! - [Implementations](#implementations)
//! - [Usage Examples](#usage-examples)
//! 
//! ## Error
//! Enumerates the different types of errors that can occur within the Resend Email Library.
//! 
//! ## Implementations
//! `Error` enum is used to represent error outcomes in email operations, particularly in the `ResendClient::send` method.
//! 
//! ## Usage Examples
//! ```rust
//! use resend_rs::Error;
//! 
//! let err = Error::ResendError("Email not sent".to_string());
//! 
//! println!("{:?}", err);
//! 
//! // Output: ResendError("Email not sent") 
//! 
//! ```
use thiserror::Error;

/// Represents all the types of errors that can occur within the Resend Email Library.
#[derive(Error, Debug)]
pub enum Error {
    /// Error type for network-related issues, wrapping the `reqwest::Error`.
    #[error("{0}")]
    ReqwestError(#[from] reqwest::Error),

    /// Error type for issues specifically related to the Resend API, such as non-success responses.
    #[error("Resend Error: {0}")]
    ResendError(String),
}