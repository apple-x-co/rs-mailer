# rs-mailer

## Usage

```bash
SMTP_HOST=example.com SMTP_PASSWORD=xxx SMTP_USER=xxx rs-mailer --config config.json
# SMTP_HOST=example.com SMTP_PASSWORD=xxx SMTP_PORT=587 SMTP_USER=xxx rs-mailer --config config.json
# SMTP_HOST=example.com SMTP_PASSWORD=xxx SMTP_PORT=587 SMTP_ENCRYPTION=starttls SMTP_USER=xxx rs-mailer --config config.json 
```

## Config

```json
{
  "$schema": "https://raw.githubusercontent.com/apple-x-co/rs-mailer/refs/heads/main/schema/schema.json",
  "from": {
    "user": "hello",
    "domain": "example.com",
    "name": "HELLO"
  },
  "to": [
    {
      "user": "hello",
      "domain": "example.com"
    }
  ],
  "subject": "This mail is TEST",
  "text": "HI!",
  "files": [
    {
      "path": "/path/to/elePHPant-clear.png",
      "name": "elePHPant-clear.png"
    }
  ]
}
```