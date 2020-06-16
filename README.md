# Lettre Email Client Hosted in AWS Lambda (Rust)

## Local Testing of the function

```sh
serverless invoke local -f personal_website_email_client -d \ '{"body": { "name":"John", "email": "john@john.com", "message": "test message"}}'
```
