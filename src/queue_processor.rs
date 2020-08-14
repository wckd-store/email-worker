use std::env::var;

use lapin::{
    Channel, Consumer, Queue,

    options::*, 
    types::FieldTable,

    Result
};

use crate::mailer::send_mail;

lazy_static! {

    static ref MESSAGES_PER_CONSUMER: u16 = { 
        var("MESSAGES_PER_CONSUMER").unwrap_or_else(|_| {
            warn!("Could not find MESSAGES_PER_CONSUMER variable, falling back to default amount");
            "10".into()
        }).parse().unwrap_or(10)
    };

}

pub async fn setup_qos(channel: &Channel) -> Result<()> {    channel.basic_qos(
        *MESSAGES_PER_CONSUMER, 
        BasicQosOptions::default()
    ).await
}

pub async fn declare_queue(queue: &str, channel: &Channel) -> Result<Queue> {
    channel.queue_declare(
        queue, 
        QueueDeclareOptions::default(), 
        FieldTable::default()
    ).await
}

pub async fn create_consumer(queue: &str, id: &str, channel: &Channel) -> Result<Consumer> {
    channel.basic_consume(
        queue, 
        id, 
        BasicConsumeOptions::default(), 
        FieldTable::default()
    ).await
}

pub async fn setup_listener(consumer: Consumer) {
    let mut iterator = consumer.into_iter();

    while let Some(delivery) = iterator.next() {
        if delivery.is_err() {
            error!("Could not read from queue, {:?}", delivery.unwrap_err());
            continue
        }

        let (channel, delivery) = delivery.unwrap();

        let body = String::from_utf8(delivery.data).unwrap();

        if let Err(err) = send_mail(body) {
            error!("Could not dispatch email, {:?}", err);
            continue
        }

        let result = channel.basic_ack(
            delivery.delivery_tag, 
            BasicAckOptions::default()
        ).await;
        
        if let Err(err) = result {
            error!("Could not acknowledge email, {:?}", err);
            continue
        }
    }
}