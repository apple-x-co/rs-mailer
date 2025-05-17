use crate::config::Config;
use anyhow::{anyhow, Result};
use lettre::message::header::ContentType;
use lettre::message::Mailbox;
use lettre::transport::smtp::authentication::Credentials;
use lettre::Address;
use lettre::Message;
use lettre::SmtpTransport;
use lettre::Transport;

pub struct Server {
    pub host: String,
    pub port: u16,
    pub encryption: String,
    pub user: String,
    pub password: String,
}

pub fn send(server: Server, config: Config) -> Result<()> {
    let mut message_builder = Message::builder();
    message_builder = message_builder.from(Mailbox::new(
        config.from.name.clone(),
        Address::new(config.from.user.clone(), config.from.domain.clone())?,
    ));
    message_builder = message_builder.reply_to(Mailbox::new(
        config.from.name.clone(),
        Address::new(config.from.user.clone(), config.from.domain.clone())?,
    ));

    for to in config.to {
        message_builder = message_builder.to(Mailbox::new(
            to.name.clone(),
            Address::new(to.user.clone(), to.domain.clone())?,
        ))
    }

    if let Some(cc) = &config.cc {
        for cc in cc {
            message_builder = message_builder.cc(Mailbox::new(
                cc.name.clone(),
                Address::new(cc.user.clone(), cc.domain.clone())?,
            ))
        }
    }

    if let Some(bcc) = &config.bcc {
        for bcc in bcc {
            message_builder = message_builder.bcc(Mailbox::new(
                bcc.name.clone(),
                Address::new(bcc.user.clone(), bcc.domain.clone())?,
            ))
        }
    }

    message_builder = message_builder.subject(config.subject.clone());
    message_builder = message_builder.header(ContentType::TEXT_PLAIN);

    // TODO: Attachments
    // xxx
    // xxx

    // TODO: HTML
    // xxx
    // xxx

    let message = message_builder.body(config.body)?;

    let creds = Credentials::new(server.user, server.password);
    let mailer = match server.encryption.as_str() {
        "starttls" => SmtpTransport::starttls_relay(&server.host)?
            .port(server.port)
            .credentials(creds)
            .build(),
        _ => SmtpTransport::relay(&server.host)?
            .port(server.port)
            .credentials(creds)
            .build(),
    };

    match mailer.send(&message) {
        Ok(_) => Ok(()),
        Err(e) => Err(anyhow!("Could not send email: {:?}", e))
    }
}
