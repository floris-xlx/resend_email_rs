
use resend_rs::ResendClient;
use resend_rs::MailText;
use resend_rs::Email;


async fn send_mail_text() {
    let mail: MailText = MailText {
        from: "Floris <floris@xylex.cloud>".to_string(),
        to: vec!["floris.trades@gmail.com".to_string()],
        subject: "Diamant Capital | Purchase confirmation".to_string(),
        text: "thank you for buying chat".to_string(),
        attachments: None,
    };

    let client: ResendClient = ResendClient::new("re_PnFfJMQA_KqkorjMeqMdcnTUee3ei4rsR".to_string());
    let resp: Email = client.send(&mail).await.unwrap();

    println!("{:?}", resp) // client::Email;
}


#[tokio::main]
async fn main() {
    send_mail_text().await;
}