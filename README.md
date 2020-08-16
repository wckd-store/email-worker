Email Worker
---

An email worker written in Rust. This application process a RabbitMQ queue and is responsible for sending them.

The queue message is structured as a JSON containing the sender, receiver, subject and the template to be used. 

The worker will look for the template in the `assets/templates` folder and replace the placeholders with the values specified in the `placeholders` JSON field. If the template is set to `custom`, the `body` field will be used instead as the HTML email body.

> Dependencies
* [Rust](https://github.com/rust-lang/rust) v1.45.2
  * [Lettre](https://github.com/lettre/lettre), the email dispatcher
  * [Lapin](https://github.com/CleverCloud/lapin), RabbitMQ client
  * [Tokio](https://github.com/tokio-rs/tokio)