use lettre::{
    smtp::{
        authentication::{Credentials, Mechanism},
        client::net::ClientTlsParameters,
        ConnectionReuseParameters, response::Response
    },
    Transport, ClientSecurity, 
    SmtpClient, SmtpTransport
};

use lettre_email::{Email, error::Error};

use std::{sync::Mutex, collections::HashMap};

use std::fs::{read_dir, read_to_string};

lazy_static! {

    static ref MAILER: Mutex<SmtpTransport> = {
        use crate::CONFIG;
        
        let address = &CONFIG.smtp_server;

        let credentials = Credentials::new(
            (&CONFIG.smtp_username).clone(), 
            (&CONFIG.smtp_password).clone()
        );

        use native_tls::{Protocol, TlsConnector};

        let tls = TlsConnector::builder()
                .min_protocol_version(Some(Protocol::Tlsv10))
                .build()
                .unwrap();

        let security = ClientSecurity::Required(
            ClientTlsParameters::new(address.clone(), tls)
        );

        Mutex::new(
            SmtpClient::new((address.as_str(), 587), security)
                .unwrap()
                .smtp_utf8(true)
                .credentials(credentials)
                .authentication_mechanism(Mechanism::Login)
                .connection_reuse(ConnectionReuseParameters::ReuseUnlimited)
                .transport()
        )
    };

    static ref TEMPLATES_MAP: HashMap<String, String> = {
        let mut map = HashMap::new();

        let dir = read_dir("assets/templates/").expect("Could not list template dir entries");

        dir.filter(| entry | entry.is_ok())
            .map(| entry | entry.unwrap())
            .for_each(| entry | {
                let name = entry.file_name().into_string();
    
                if name.is_err() {
                    return
                }
    
                if let Ok(value) = read_to_string(entry.path()) {
                    let name = name.unwrap();
                    let name = name.chars().take(name.len() - 5).collect::<String>();

                    map.insert(name, value);
                }
            });

        map
    };

}

pub fn list_templates() {
    TEMPLATES_MAP.keys().for_each(| key | {
        info!("Known template: {}", key);
    });
}

pub fn send_mail(email: JsonEmail) -> Result<Response, Box<dyn std::error::Error>> {
    let email = email.to_sendable_email()?;

    let mut mailer = MAILER.lock().unwrap();

    let result = mailer.send(email.into())?;

    mailer.close();

    Ok(result)
}

use std::io::{Error as IoError, ErrorKind};

use serde_json::{from_slice, Result as SerdeResult};

#[derive(Serialize, Deserialize, Debug)]
pub struct JsonEmail {

    pub template: String,

    pub from: String,
    pub to: Vec<String>,

    pub subject: String,

    pub body: Option<String>,
    pub placeholders: Option<HashMap<String, String>>

}

impl JsonEmail {

    pub fn from_slice(json: &[u8]) -> SerdeResult<Self> {
        from_slice(json)
    }

    pub fn to_sendable_email(&self) -> Result<Email, Error> {
        let mut builder = Email::builder()
                                        .from(self.from.as_str())
                                        .subject(self.subject.as_str());

        for addr in self.to.iter() {
            builder = builder.to(addr.as_str());
        }

        let template = self.template.to_lowercase();

        if template == "custom" {
            match &self.body {
                Some(body) => builder = builder.html(body.as_str()),
                None => return create_error("Invalid body")
            }

            return builder.build()
        }

        match TEMPLATES_MAP.get(&template) {
            Some(body) => {
                match &self.placeholders {
                    Some(placeholders) => {
                        let mut body = body.clone();

                        placeholders.iter()
                                    .for_each(| (key, value) | {
                                        body = body.replace(key, value);
                                    });
                        
                        builder = builder.html(body);
                    },

                    None => return create_error("Invalid placeholders")
                }
                
                builder = builder.html(body.as_str());
            },

            None => return create_error("Invalid template")
        }

        builder.build()
    }

}

fn create_error(error: &str) -> Result<Email, Error> {
    Err(Error::Io(IoError::new(ErrorKind::NotFound, error)))
}