use lettre::smtp::authentication::{Credentials, Mechanism};
use lettre::smtp::client::net::ClientTlsParameters;

use lettre::smtp::extension::ClientId;
use lettre::smtp::ConnectionReuseParameters;

use lettre::{Transport, SmtpClient, ClientSecurity, SmtpTransport};

use lettre_email::Email;

use native_tls::{Protocol, TlsConnector};

use std::env::var;

use std::sync::Mutex;

lazy_static! {

    static ref MAILER: Mutex<SmtpTransport> = {
        let address = var("SMTP_SERVER").unwrap_or_else(|_| {
            warn!("Could not find SMTP_SERVER variable, falling back to development server");
            "smtp.mailtrap.io".into()
        });

        let credentials = Credentials::new(
            var("SMTP_USERNAME").unwrap_or_else(|_| {
                warn!("Could not find SMTP_USERNAME variable, falling back to default username");
                "username".into()
            }), 
            var("SMTP_PASSWORD").unwrap_or_else(|_| {
                warn!("Could not find SMTP_PASSWORD variable, falling back to default password");
                "password".into()
            })
        );

        let tls = TlsConnector::builder()
                .min_protocol_version(Some(Protocol::Tlsv10))
                .build()
                .unwrap();

        let security = ClientSecurity::Required(
            ClientTlsParameters::new(address.clone(), tls)
        );

        Mutex::new(
            SmtpClient::new((address.as_str(), 465), security)
                .unwrap()
                .smtp_utf8(true)
                .credentials(credentials)
                .authentication_mechanism(Mechanism::Login)
                .connection_reuse(ConnectionReuseParameters::ReuseUnlimited)
                .transport()
        )
    };

}

pub fn send_mail() {
    let email = Email::builder()
            .from("")
            .to("")
            .body("")
            .build()
            .unwrap();

    let mut mailer = MAILER.lock().unwrap();

    if let Err(err) = mailer.send(email.into()) {
        error!("{:?}", err)
    }
    
    mailer.close();
}