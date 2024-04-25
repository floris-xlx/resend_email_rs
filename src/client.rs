//! # Resend Client Module
//!
//! This module provides functionalities to send emails using the Resend API.
//!
//! ## Table of Contents
//! - [Structs](#structs)
//! - [ResendClient](#resendclient)
//! - [Implementations](#implementations)
//! - [ResendClient::new](#resendclientnew)
//! - [ResendClient::send](#resendclientsend)
//! - [Usage Examples](#usage-examples)
//! - [Creating a Client](#creating-a-client)
//! - [Sending an Email](#sending-an-email)
//! - [Errors](#errors)
//! - [Success Types](#success-types)
//!
//! ## Structs
//! ### ResendClient
//! Represents a client that can send emails.
//!
//! ## Implementations
//! ### ResendClient::new
//! Creates a new instance of `ResendClient`.
//!
//! ### ResendClient::send
//! Sends an email.
//! Returns a `Result<Email, Error>` indicating the outcome of the send operation.
//!
//! ## Usage Examples
//! ### Creating a Client
//! ```
//! let client = ResendClient::new("your_auth_token".to_string());
//! ```
//!
//! ### Sending an Email
//! ```
//! let email = client.send(&mail).await?;
//! ```
//!
//! ## Errors
//! - `Error::ReqwestError` - If there is a network-related error.
//! - `Error::ResendError` - If the API returns a non-success status.
//!
//! ## Success Types
//! - `Success::EmailSent` - Indicates that the email was sent successfully.

use serde::Serialize;
use reqwest::{
    Client, 
    Response, 
    header::CONTENT_TYPE
};

use crate::{
    API_URL, 
    Email, 
    ResendClient, 
    error::Error,
    success::Success
};

impl ResendClient {
    /// Creates a new `ResendClient` with the given authentication token.
    ///
    /// # Arguments
    /// * `auth_token` - A `String` containing the authentication token.
    ///
    /// # Returns
    /// A new instance of `ResendClient`.
    pub fn new(auth_token: String) -> Self {
        ResendClient { auth_token }
    }

    /// Sends an email using the Resend API.
    ///
    /// # Arguments
    /// * `mail` - A reference to an object that implements `Serialize`, representing the email to send.
    ///
    /// # Returns
    /// A `Result<Email, Error>`:
    /// - `Ok(Email)` if the email was sent successfully.
    /// - `Err(Error)` if there was an error sending the email.
    pub async fn send(&self, mail: &impl Serialize) -> Result<Email, Error> {
        let resp: Response = Client::new()
            .post(API_URL)
            .bearer_auth(self.auth_token.to_string())
            .header(CONTENT_TYPE, "application/json")
            .json(mail)
            .send()
            .await
            .unwrap();

        match resp.status().is_success() {
            true => {
                let email: Email = resp.json().await.unwrap();
                println!("{}", Success::EmailSent);
                Ok(email)
            }
            false => Err(Error::ResendError(resp.text().await.unwrap())),
        }
    }
}
