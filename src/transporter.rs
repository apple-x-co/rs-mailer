use anyhow::{Result, anyhow};
use lettre::Transport;
use crate::config::Config;

pub struct Server {
    pub host: String,
    pub port: u16,
    pub encryption: String,
    pub user: String,
    pub password: String,
}

pub fn send(server: Server, config: Config) -> Result<()> {
    let mut message_builder = lettre::Message::builder();
    message_builder = message_builder.from(lettre::message::Mailbox::new(
        config.from.name.clone(),
        lettre::Address::new(config.from.user.clone(), config.from.domain.clone())?,
    ));
    message_builder = message_builder.reply_to(lettre::message::Mailbox::new(
        config.from.name.clone(),
        lettre::Address::new(config.from.user.clone(), config.from.domain.clone())?,
    ));

    for to in config.to {
        message_builder = message_builder.to(lettre::message::Mailbox::new(
            to.name.clone(),
            lettre::Address::new(to.user.clone(), to.domain.clone())?,
        ))
    }

    if let Some(cc) = &config.cc {
        for cc in cc {
            message_builder = message_builder.cc(lettre::message::Mailbox::new(
                cc.name.clone(),
                lettre::Address::new(cc.user.clone(), cc.domain.clone())?,
            ))
        }
    }

    if let Some(bcc) = &config.bcc {
        for bcc in bcc {
            message_builder = message_builder.bcc(lettre::message::Mailbox::new(
                bcc.name.clone(),
                lettre::Address::new(bcc.user.clone(), bcc.domain.clone())?,
            ))
        }
    }

    message_builder = message_builder.subject(config.subject.clone());
    message_builder = message_builder.header(lettre::message::header::ContentType::TEXT_PLAIN);

    // TODO: Attachments
    // xxx
    // xxx

    // TODO: HTML
    // xxx
    // xxx

    let message = message_builder.body(config.body)?;

    let creds = lettre::transport::smtp::authentication::Credentials::new(server.user, server.password);
    let mailer = match server.encryption.as_str() {
        "starttls" => lettre::SmtpTransport::starttls_relay(&server.host)?
            .port(server.port)
            .credentials(creds)
            .build(),
        _ => lettre::SmtpTransport::relay(&server.host)?
            .port(server.port)
            .credentials(creds)
            .build(),
    };

    match mailer.send(&message) {
        Ok(_) => Ok(()),
        Err(e) => Err(anyhow!("Could not send email: {:?}", e))
    }
}
