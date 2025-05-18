use crate::config::Config;
use anyhow::{Result, anyhow};
use lettre::Address;
use lettre::Message;
use lettre::SmtpTransport;
use lettre::Transport;
use lettre::message::header::ContentType;
use lettre::message::{Attachment, Mailbox, MultiPart, SinglePart};
use lettre::transport::smtp::authentication::Credentials;
use std::fs;
use std::io::{BufReader, Read};

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

    let mut attachments: Vec<SinglePart> = Vec::new();
    if let Some(files) = config.files {
        for file in files {
            if let Some(media_type) = file.media_type {
                let attachment = Attachment::new(file.name.clone()).body(
                    fs::read(file.path.clone())?,
                    ContentType::parse(&media_type)?,
                );

                attachments.push(attachment);

                continue;
            }

            let attachment_file = fs::File::open(file.path.clone()).map_err(|e| anyhow!(e))?;

            let mut buf_reader = BufReader::new(attachment_file);
            let mut buffer = [0; 24];
            buf_reader.read(&mut buffer)?;

            if let Some(t) = infer::get(&buffer) {
                let attachment = Attachment::new(file.name.clone()).body(
                    fs::read(file.path.clone())?,
                    ContentType::parse(t.mime_type())?,
                );

                attachments.push(attachment);
            }
        }
    }

    let message = if let Some(html) = config.html {
        if attachments.len() > 0 {
            let mut multipart = MultiPart::alternative().singlepart(
                SinglePart::builder()
                    .header(ContentType::TEXT_PLAIN)
                    .body(config.body),
            );

            multipart = multipart.singlepart(
                SinglePart::builder()
                    .header(ContentType::TEXT_HTML)
                    .body(html),
            );
            
            let mut mixedpart = MultiPart::mixed().multipart(multipart);

            for attachment in attachments {
                mixedpart = mixedpart.singlepart(attachment);
            }

            message_builder.multipart(mixedpart)?
        } else {
            let mut multipart = MultiPart::alternative().singlepart(
                SinglePart::builder()
                    .header(ContentType::TEXT_PLAIN)
                    .body(config.body),
            );

            multipart = multipart.singlepart(
                SinglePart::builder()
                    .header(ContentType::TEXT_HTML)
                    .body(html),
            );

            message_builder.multipart(multipart)?
        }
    } else {
        if attachments.len() > 0 {
            let mut multipart = MultiPart::mixed().singlepart(
                SinglePart::builder()
                    .header(ContentType::TEXT_PLAIN)
                    .body(config.body),
            );

            for attachment in attachments {
                multipart = multipart.singlepart(attachment);
            }

            message_builder.multipart(multipart)?
        } else {
            message_builder
                .header(ContentType::TEXT_PLAIN)
                .body(config.body)?
        }
    };

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
        Err(e) => Err(anyhow!("Could not send email: {:?}", e)),
    }
}
