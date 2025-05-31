# rs-mailer

Rustで書かれたシンプルで高性能なコマンドライン メールクライアント。JSONファイルでメール設定を行い、テキスト・HTML形式のメール送信、添付ファイルの送信に対応しています。

## 特徴

- 📧 **シンプルなJSON設定**: メール内容をJSONファイルで管理
- 🔒 **セキュアな認証**: SMTP認証をサポート（STARTTLS/TLS対応）
- 📎 **添付ファイル対応**: 複数ファイルの自動MIME-Type判定
- 🎨 **マルチフォーマット**: プレーンテキスト・HTML両対応
- ⚡ **高性能**: Rustによる高速・安全な実装
- 🛡️ **バリデーション**: JSON Schemaによる設定ファイルの検証

## 使用方法

### 環境変数の設定

メール送信には以下の環境変数が必要です：

```bash
export SMTP_HOST="smtp.gmail.com"
export SMTP_USER="your-email@gmail.com"
export SMTP_PASSWORD="your-app-password"
export SMTP_PORT="587"                    # オプション（デフォルト: 25）
export SMTP_ENCRYPTION="starttls"         # オプション（none/tls/starttls）
```

### 設定ファイルの作成

メール内容をJSONファイルで定義します：

```json
{
  "$schema": "https://raw.githubusercontent.com/apple-x-co/rs-mailer/refs/heads/main/schema/schema.json",
  "from": {
    "user": "sender",
    "domain": "example.com",
    "name": "送信者名"
  },
  "to": [
    {
      "user": "recipient",
      "domain": "example.com", 
      "name": "受信者名"
    }
  ],
  "subject": "テストメール",
  "text": "こんにちは！\nプレーンテキストのメッセージです。"
}
```

### メール送信

```bash
rs-mailer --config mail.json
```

または短縮形：

```bash
rs-mailer -c mail.json
```

## 設定ファイルの詳細

### 基本構造

```json
{
  "$schema": "https://raw.githubusercontent.com/apple-x-co/rs-mailer/refs/heads/main/schema/schema.json",
  "from": {
    "user": "送信者のユーザー名",
    "domain": "送信者のドメイン",
    "name": "送信者の表示名（オプション）"
  },
  "to": [
    {
      "user": "受信者のユーザー名", 
      "domain": "受信者のドメイン",
      "name": "受信者の表示名（オプション）"
    }
  ],
  "cc": [
    {
      "user": "CCのユーザー名",
      "domain": "CCのドメイン", 
      "name": "CCの表示名（オプション）"
    }
  ],
  "bcc": [
    {
      "user": "BCCのユーザー名",
      "domain": "BCCのドメイン",
      "name": "BCCの表示名（オプション）"
    }
  ],
  "subject": "メールの件名",
  "text": "プレーンテキストの本文",
  "html": "<h1>HTMLの本文</h1><p>HTMLメールです。</p>",
  "files": [
    {
      "name": "document.pdf",
      "path": "/path/to/document.pdf",
      "media_type": "application/pdf"
    }
  ]
}
```

### フィールドの説明

| フィールド | 必須 | 説明 |
|-----------|------|------|
| `from` | ✅ | 送信者の情報 |
| `to` | ✅ | 受信者のリスト（1人以上） |
| `cc` | ❌ | CCのリスト |
| `bcc` | ❌ | BCCのリスト |
| `subject` | ✅ | メールの件名（1-100文字） |
| `text` | ✅ | プレーンテキストの本文 |
| `html` | ❌ | HTMLの本文 |
| `files` | ❌ | 添付ファイルのリスト |

### 添付ファイル

添付ファイルは自動でMIME-Typeが判定されますが、明示的に指定することも可能です：

```json
{
  "files": [
    {
      "name": "report.pdf",
      "path": "/home/user/documents/report.pdf"
    },
    {
      "name": "image.jpg", 
      "path": "/home/user/images/photo.jpg",
      "media_type": "image/jpeg"
    }
  ]
}
```

## 使用例

### シンプルなテキストメール

```json
{
  "from": {
    "user": "info",
    "domain": "company.com",
    "name": "お知らせ"
  },
  "to": [
    {
      "user": "customer",
      "domain": "example.com",
      "name": "お客様"
    }
  ],
  "subject": "重要なお知らせ",
  "text": "いつもご利用いただき、ありがとうございます。\n\n重要な変更についてお知らせいたします。"
}
```

### HTMLメール + 添付ファイル

```json
{
  "from": {
    "user": "newsletter",
    "domain": "company.com", 
    "name": "ニュースレター"
  },
  "to": [
    {
      "user": "subscriber",
      "domain": "example.com"
    }
  ],
  "subject": "月間レポート - 2024年5月",
  "text": "月間レポートをお送りします。\n\n詳細は添付のPDFファイルをご確認ください。",
  "html": "<h1>月間レポート</h1><p>詳細は<strong>添付ファイル</strong>をご確認ください。</p>",
  "files": [
    {
      "name": "monthly_report_2024_05.pdf",
      "path": "./reports/monthly_report_2024_05.pdf"
    }
  ]
}
```

### 複数受信者への一斉送信

```json
{
  "from": {
    "user": "admin",
    "domain": "company.com",
    "name": "システム管理者"
  },
  "to": [
    {
      "user": "user1",
      "domain": "company.com",
      "name": "ユーザー1"
    },
    {
      "user": "user2", 
      "domain": "company.com",
      "name": "ユーザー2"
    }
  ],
  "cc": [
    {
      "user": "manager",
      "domain": "company.com",
      "name": "マネージャー"
    }
  ],
  "subject": "システムメンテナンスのお知らせ",
  "text": "システムメンテナンスを実施いたします。\n\n日時: 2024年6月1日 2:00-4:00\n影響: 全サービス停止"
}
```

## SMTP設定例

### Gmail
```bash
export SMTP_HOST="smtp.gmail.com"
export SMTP_PORT="587"
export SMTP_ENCRYPTION="starttls"
export SMTP_USER="your-email@gmail.com"
export SMTP_PASSWORD="your-app-password"  # アプリパスワードを使用
```

### Outlook/Hotmail
```bash
export SMTP_HOST="smtp-mail.outlook.com"
export SMTP_PORT="587"
export SMTP_ENCRYPTION="starttls"
export SMTP_USER="your-email@outlook.com"
export SMTP_PASSWORD="your-password"
```

### 独自SMTPサーバー
```bash
export SMTP_HOST="mail.your-domain.com"
export SMTP_PORT="25"
export SMTP_ENCRYPTION="none"
export SMTP_USER="username"
export SMTP_PASSWORD="password"
```

## トラブルシューティング

### よくあるエラー

**環境変数が設定されていない**
```
Error: environment variable not found
```
→ 必要な環境変数（SMTP_HOST, SMTP_USER, SMTP_PASSWORD）を設定してください。

**JSON形式エラー**
```
Error: JSON validation failed
```
→ 設定ファイルのJSON形式を確認してください。必須フィールドが不足している可能性があります。

**添付ファイルが見つからない**
```
Error: No such file or directory
```
→ 添付ファイルのパスが正しいか確認してください。

**SMTP認証エラー**
```
Error: Could not send email: Authentication failed
```
→ SMTP認証情報（ユーザー名・パスワード）を確認してください。
