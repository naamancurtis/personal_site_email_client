# Lettre Email Client (Rust) Hosted in AWS Lambda

I use the code in this repo to forward emails from the contact me section of my [portfolio](naamancurtis.com) to myself.

## ENV Var

| Key         | Description                                                                                         |
| ----------- | --------------------------------------------------------------------------------------------------- |
| DESTINATION | Email address receiving the emails                                                                  |
| USERNAME    | Username used to sign into the mail client                                                          |
| PASSWORD    | Password used to sign into the mail client                                                          |
| MAIL_CLIENT | The third party client you're using to send the emails                                              |
| ORIGIN      | URL you want the requests to come from _(Assuming you want to restrict who can trigger the lambda)_ |

## Bugs, Feedback or Improvements

There's not a lot of examples/documentation out there on using `aws-lambda runtime` or `aws-lambda http`, so if you have any improvements
or recommendation's I'd really appreciate it!
