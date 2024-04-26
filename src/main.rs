
// use resend_email_rs::ResendClient;
// use resend_email_rs::MailText;
// use resend_email_rs::Email;


// async fn send_mail_text() {
//     let mail: MailText = MailText {
//         from: "Floris <floris@xylex.cloud>".to_string(),
//         to: vec!["floris.xx@gmail.com".to_string()],
//         subject: "Purchase confirmation".to_string(),
//         text: "thank you for buying".to_string(),
//         attachments: None,
//     };

//     let client: ResendClient = ResendClient::new("xx".to_string());
//     let resp: Email = client.send(&mail).await.unwrap();

//     println!("{:?}", resp) // client::Email;
// }


// #[tokio::main]
// async fn main() {
//     send_mail_text().await;
// }

fn main() {
    println!("Hello, world!");
}