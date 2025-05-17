# rs-sendmail

Config

```json
{
  "$schema": "./schema/schema.json",
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
  "body": "HI!"
}
```

Run

```bash
SMTP_HOST=example.com SMTP_PASSWORD=xxx SMTP_PORT=587 SMTP_USER=xxx rs-sendmail --config config.json 
```
