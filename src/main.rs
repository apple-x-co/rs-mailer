mod config;

use anyhow::Result;
use clap::Parser;
use lettre::Transport;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    config: String,
}

fn main() -> Result<()> {
    let smtp_host = std::env::var("SMTP_HOST")?;
    let smtp_port = std::env::var("SMTP_PORT").unwrap_or("25".to_string()); // NOTE: 25,465,587
    let smtp_encryption = std::env::var("SMTP_ENCRYPTION").unwrap_or("none".to_string()); // NOTE: none,tls,starttls
    let smtp_user = std::env::var("SMTP_USER")?;
    let smtp_password = std::env::var("SMTP_PASSWORD")?;

    let args = Args::parse();

    let config = config::parse(&args.config)?;

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

    // TODO: Attachments, HTML
    let message = message_builder.body(config.body)?;

    let creds = lettre::transport::smtp::authentication::Credentials::new(smtp_user, smtp_password);
    let mailer = match smtp_encryption.as_str() {
        "starttls" => lettre::SmtpTransport::starttls_relay(&smtp_host)?
            .port(smtp_port.parse::<u16>()?)
            .credentials(creds)
            .build(),
        _ => lettre::SmtpTransport::relay(&smtp_host)?
            .port(smtp_port.parse::<u16>()?)
            .credentials(creds)
            .build(),
    };

    match mailer.send(&message) {
        Ok(_) => println!("Email sent successfully!"),
        Err(e) => eprintln!("Could not send email: {:?}", e),
    };

    Ok(())
}
