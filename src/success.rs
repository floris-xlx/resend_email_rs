//! # Success Types Module
//!
//! This module defines the various success types returned by operations within the Resend email service.
//!
//! ### Table of Contents
//! - [Enums](#enums)
//! - [Success](#success)
//! - [Implementations](#implementations)
//! - [Usage Examples](#usage-examples)
//!
//! ### Success
//! Enumerates the different types of success outcomes for email operations.
//!
//! ## Implementations
//! `Success` enum is used to represent successful outcomes in email operations, particularly in the `ResendClient::send` method.
//!
//! ## Usage Examples
//! ### Checking for a Successful Email Send
//! ```rust
//! match resend_client.send(&email).await {
//!     Ok(success) => match success {
//!         Success::EmailSent => println!("Email was successfully sent."),
//!     },
//!     Err(e) => println!("Failed to send email: {}", e),
//! }
//! ```
//!
//! ## Returns Success
//! - `Success::EmailSent` - Indicates that the email was sent successfully.
//!
//! ## Returns Errors
//! This module itself does not return errors directly, but is used in contexts where errors may be returned by other components.
use thiserror::Error;


/// ## Enumerates the different types of success outcomes for email operations.
/// 
/// ### Fields
/// - `EmailSent` - Indicates that the email was sent successfully.
/// 
/// ### Usage
/// `Success` enum is used to represent successful outcomes in email operations, particularly in the `ResendClient::send` method.
#[derive(Error, Debug)]
pub enum Success {
    #[error("Email was successfully sent")]
    EmailSent,
}
