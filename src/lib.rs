//! # Resend Email Library
//!
//! The Resend Email Library provides comprehensive solutions for email operations within Rust applications. This crate supports various email functionalities, including plain text and HTML emails, and facilitates handling attachments.
//!
//! ## Getting Started
//!
//! To use the Resend Email Library, add it to your `Cargo.toml`:
//!
//! ```toml
//! [dependencies]
//! resend_rs = "0.1.0"
//! ```
//!
//! ## Features
//! 
//! - [Authenticating the client](#authenticating-the-client).
//! - [Send plain text emails](#sending-a-plain-text-email).
//! - [Send HTML formatted emails](#sending-an-html-email).
//! - [Support for email attachments](#sending-an-email-with-attachments).
//! - [Robust error handling for network and API errors](#handling-success-and-errors).
//!
//! ## Examples
//!
//! Here are examples of how to send different types of emails using this library:
//!
//! 
//! ### Authenticating the client
//! ```rust
//! use resend_rs::ResendClient;
//! 
//! let client = ResendClient::new("your_auth_token".to_string());
//! ```
//! Make sure to replace `your_auth_token` with your actual Resend API token.
//! 
//! 
//! ### Sending a Plain Text Email
//!
//! ```rust
//! use resend_rs::{ResendClient, MailText};
//!
//! let client = ResendClient::new("your_auth_token".to_string());
//! 
//! let mail = MailText {
//!     from: "floris@xylex.ai",
//!     to: vec!["recipient@example.com"],
//!     subject: "Test Email",
//!     text: "Hello, this is a test email.",
//!     attachments: None,
//! };
//! let email_sent_status: Email = client.send(&mail).await.unwrap();
//! ```
//!
//! ### Sending an HTML Email
//!
//! ```rust
//! use resend_rs::{ResendClient, MailHtml};
//!
//! let client = ResendClient::new("your_auth_token".to_string());
//! let mail = MailHtml {
//!     from: "floris@xylex.ai",
//!     to: vec!["recipient@example.com"],
//!     subject: "Hello World",
//!     html: "<h1>Welcome</h1><p>This is an HTML email.</p>",
//!     attachments: None,
//! };
//! let email_sent_status: Email = client.send(&mail).await.unwrap();
//! ```
//!
//! ### Sending an Email with Attachments
//!
//! ```rust
//! use resend_rs::{ResendClient, MailText, Attachment};
//!
//! let client = ResendClient::new("your_auth_token".to_string());
//! let attachment = Attachment {
//!     content: vec![0, 1, 2, 3],
//!     filename: "example.txt",
//! };
//! let mail = MailText {
//!     from: "floris@xylex.ai",
//!     to: vec!["recipient@example.com"],
//!     subject: "Test Email with Attachment",
//!     text: "Please find the attachment.",
//!     attachments: Some(vec![attachment]),
//! };
//! let email_sent_status: Email = client.send(&mail).await.unwrap();
//! ```
//!
//! ## Handling Success and Errors
//!
//! The library provides detailed feedback for operations:
//!
//! - **[Success Types](success/index.html):**
//!   - [`Success::EmailSent`](success/index.html#emailsent): Indicates that the email was successfully sent.
//!
//! - **[Error Types](error/index.html):**
//!   - [`Error::ReqwestError`](error/index.html#reqwesterror): If there is a network-related error.
//!   - [`Error::ResendError`](error/index.html#resenderror): If the API returns a non-success status.
//!
//! Every email send operation returns a `Result<Email, Error>` indicating the outcome, which makes it easy to handle success or diagnose issues programmatically.

use serde_derive::{Deserialize, Serialize};

pub mod error;
pub mod success;
pub mod client;
pub mod tests;

const API_URL: &str = "https://api.resend.com/emails";

/// ## Email Struct
/// Represents an email object with an identifier.
/// 
/// ### Fields
/// - `id` - A `String` representing the email identifier.
/// 
/// ### Examples
/// ```rust
/// 
/// let email = Email {
///   id: "email_id".to_string(),
/// };
/// ```
#[derive(Debug, Deserialize)]
pub struct Email {
    pub id: String,
}

/// ## ResendClient Struct
/// Represents a client that can send emails using an authentication token.
/// 
/// ### Fields
/// - `auth_token` - A `String` representing the authentication token.
/// 
/// ### Examples
/// ```rust
/// 
/// let client = ResendClient {
///    auth_token: "your_auth_token".to_string(),
/// };
#[derive(Debug, Serialize)]
pub struct ResendClient {
    auth_token: String,
}

/// ## Attachment Struct
/// Represents an email attachment with content and filename.
/// 
/// ### Fields
/// - `content` - A `Vec<u8>` representing the content of the attachment.
/// - `filename` - A `String` representing the filename of the attachment.
/// 
/// ### Examples
/// ```rust
/// 
/// let attachment = Attachment {
///    content: vec![0, 1, 2, 3],
///    filename: "example.txt",
/// };
/// ```
/// 
#[derive(Debug, Serialize)]
pub struct Attachment {
    pub content: Vec<u8>,
    pub filename: String,
}


/// ## MailText Struct
/// Represents a plain text email with optional attachments.
/// 
/// ### Fields
/// - `from` - A `String` representing the sender's email address.
/// - `to` - A `Vec<String>` representing the recipient's email addresses.
/// - `subject` - A `String` representing the email subject.
/// - `text` - A `String` representing the text content of the email.
/// - `attachments` - An optional `Vec<Attachment>` representing email attachments.
/// 
/// ### Examples
/// ```rust
/// 
/// let mail = MailText {
///     from: "floris@xylex.ai",
///     to: vec!["person@example.com"]
///     subject: "Test Email",
///     text: "Hello, this is a test email.",
///     attachments: None,
/// };
/// ```
/// 
/// 
#[derive(Debug, Serialize)]
pub struct MailText {
    pub from: String,
    pub to: Vec<String>,
    pub subject: String,
    pub text: String,
    pub attachments: Option<Vec<Attachment>>,
}


/// ## MailHtml Struct
/// Represents an HTML email with optional attachments.
/// 
/// ### Fields
/// - `from` - A `String` representing the sender's email address.
/// - `to` - A `Vec<String>` representing the recipient's email addresses.
/// - `subject` - A `String` representing the email subject.
/// - `html` - A `String` representing the HTML content of the email.
/// - `attachments` - An optional `Vec<Attachment>` representing email attachments.
/// 
/// ### Examples
/// ```rust
/// 
/// let mail = MailHtml {
///     from: "floris@xylex.ai",
///     to: vec!["person@example.com"],
///     subject: "Test Email",
///     html: "<h1>Hello</h1><p>This is a test email.</p>",
///     attachments: None,
/// };
#[derive(Debug, Serialize)]
pub struct MailHtml {
    pub from: String,
    pub to: Vec<String>,
    pub subject: String,
    pub html: String,
    pub attachments: Option<Vec<Attachment>>,
}