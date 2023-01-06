use lettre::{
    message::header::ContentType, transport::smtp::authentication::Credentials, Message,
    SmtpTransport, Transport,
};

use super::models::{Person, Task};

/// Sends an email to an SMTPS server on port 465.
///
/// # Parameters
///
/// * `host` is the SMTP server name.
/// * `username` is the sender's username used for authentication.
/// * `password` is the sender's password used for authentication.
/// * `from` is the source address.
/// * `to` is the destination address.
/// * `subject` is the email's subject.
/// * `body` is the email's content.
pub fn send_email(
    host: &str,
    username: &str,
    password: &str,
    from: &str,
    to: &str,
    subject: String,
    body: String,
) -> Result<<SmtpTransport as Transport>::Ok, <SmtpTransport as Transport>::Error> {
    let email = Message::builder()
        .from(from.parse().unwrap())
        .to(to.parse().unwrap())
        .subject(subject)
        .header(ContentType::TEXT_PLAIN) // Allows UTF-8.
        .body(body)
        .unwrap();

    let credentials = Credentials::new(username.to_owned(), password.to_owned());

    let mailer = SmtpTransport::relay(host)
        .unwrap()
        .credentials(credentials)
        .build();

    mailer.send(&email)
}

/// Writes the email's subject according to the person and its task.
pub fn generate_email_subject(_person: &Person, task: &Task) -> String {
    format!("Ta tâche de cette semaine ({})", task.name)
}

/// Writes the email's body according to the person and its task.
pub fn generate_email_body(person: &Person, task: &Task) -> String {
    format!(
        "Bonjour {},\n\
        \n\
        Cette semaine, ta tâche est \"{}\".\n\
        \n\
        Cordialement,\n\
        TasKot v0.1.1",
        person.name, task.name,
    )
}
