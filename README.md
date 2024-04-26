# Resend Email Library

The Resend Email Library provides comprehensive solutions for email operations within Rust applications. This crate supports various email functionalities, including plain text and HTML emails, and facilitates handling attachments.

## Coming soon
I will add the bulk emailing method soon :)

## Getting Started

To use the Resend Email Library, add it to your `Cargo.toml`:

```toml
[dependencies]
resend_rs = "0.1.0"
```

## Features

- [Authenticating the client](#authenticating-the-client).
- [Send plain text emails](#sending-a-plain-text-email).
- [Send HTML formatted emails](#sending-an-html-email).
- [Support for email attachments](#sending-an-email-with-attachments).
- [Robust error handling for network and API errors](#handling-success-and-errors).

## Examples

Here are examples of how to send different types of emails using this library:


### Authenticating the client
```rust
use resend_rs::ResendClient;

let client = ResendClient::new("your_auth_token".to_string());
```
Make sure to replace `your_auth_token` with your actual Resend API token.


### Sending a Plain Text Email

```rust
use resend_rs::{ResendClient, MailText};

let client = ResendClient::new("your_auth_token".to_string());

let mail = MailText {
    from: "floris@xylex.ai",
    to: vec!["recipient@example.com"],
    subject: "Test Email",
    text: "Hello, this is a test email.",
    attachments: None,
};
let email_sent_status: Email = client.send(&mail).await.unwrap();
```

### Sending an HTML Email

```rust
use resend_rs::{ResendClient, MailHtml};

let client = ResendClient::new("your_auth_token".to_string());
let mail = MailHtml {
    from: "floris@xylex.ai",
    to: vec!["recipient@example.com"],
    subject: "Hello World",
    html: "<h1>Welcome</h1><p>This is an HTML email.</p>",
    attachments: None,
};
let email_sent_status: Email = client.send(&mail).await.unwrap();
```

### Sending an Email with Attachments

```rust
use resend_rs::{ResendClient, MailText, Attachment};

let client = ResendClient::new("your_auth_token".to_string());
let attachment = Attachment {
    content: vec![0, 1, 2, 3],
    filename: "example.txt",
};
let mail = MailText {
    from: "floris@xylex.ai",
    to: vec!["recipient@example.com"],
    subject: "Test Email with Attachment",
    text: "Please find the attachment.",
    attachments: Some(vec![attachment]),
};
let email_sent_status: Email = client.send(&mail).await.unwrap();
```

## Handling Success and Errors

The library provides detailed feedback for operations:

- **[Success Types](success/index.html):**
- [`Success::EmailSent`](success/index.html#emailsent): Indicates that the email was successfully sent.

- **[Error Types](error/index.html):**
- [`Error::ReqwestError`](error/index.html#reqwesterror): If there is a network-related error.
- [`Error::ResendError`](error/index.html#resenderror): If the API returns a non-success status.

Every email send operation returns a `Result<Email, Error>` indicating the outcome, which makes it easy to handle success or diagnose issues programmatically.