use crate::config::Config;
use anyhow::{Result, anyhow};
use lettre::Address;
use lettre::Message;
use lettre::SmtpTransport;
use lettre::Transport;
use lettre::message::header::ContentType;
use lettre::message::{Attachment, Mailbox, MessageBuilder, MultiPart, SinglePart};
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

    let message = build_message(message_builder, config.text, config.html, attachments)?;

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

fn build_message(
    builder: MessageBuilder,
    text_body: String,
    html_body: Option<String>,
    attachments: Vec<SinglePart>,
) -> Result<Message> {
    // プレーンテキスト部分を作成
    let plain_text_part = SinglePart::builder()
        .header(ContentType::TEXT_PLAIN)
        .body(text_body);

    // 添付ファイルの有無とHTMLの有無で4つのケースを処理
    match (html_body, !attachments.is_empty()) {
        // Case 1: HTMLあり、添付ファイルあり
        (Some(html), true) => {
            let html_part = SinglePart::builder()
                .header(ContentType::TEXT_HTML)
                .body(html);

            let alternative_part = MultiPart::alternative()
                .singlepart(plain_text_part)
                .singlepart(html_part);

            let mut mixed_part = MultiPart::mixed().multipart(alternative_part);

            // 添付ファイルを追加
            for attachment in attachments {
                mixed_part = mixed_part.singlepart(attachment);
            }

            Ok(builder.multipart(mixed_part)?)
        }

        // Case 2: HTMLあり、添付ファイルなし
        (Some(html), false) => {
            let html_part = SinglePart::builder()
                .header(ContentType::TEXT_HTML)
                .body(html);

            Ok(builder.multipart(
                MultiPart::alternative()
                    .singlepart(plain_text_part)
                    .singlepart(html_part),
            )?)
        }

        // Case 3: HTMLなし、添付ファイルあり
        (None, true) => {
            let mut mixed_part = MultiPart::mixed().singlepart(plain_text_part);

            // 添付ファイルを追加
            for attachment in attachments {
                mixed_part = mixed_part.singlepart(attachment);
            }

            Ok(builder.multipart(mixed_part)?)
        }

        // Case 4: HTMLなし、添付ファイルなし
        (None, false) => Ok(builder.singlepart(plain_text_part)?),
    }
}
