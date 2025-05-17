mod config;
mod transporter;

use anyhow::Result;
use clap::Parser;

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

    let server = transporter::Server {
        host: smtp_host,
        port: smtp_port.parse::<u16>()?,
        encryption: smtp_encryption,
        user: smtp_user,
        password: smtp_password
    };
    let config = config::parse(&args.config)?;

    transporter::send(server, config)
}
