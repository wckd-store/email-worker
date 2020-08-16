use lapin::{
    Channel, Consumer, Queue,

    options::*, 
    types::FieldTable,

    Result
};

use crate::{CONFIG, mailer::{JsonEmail, send_mail}};

pub async fn setup_qos(channel: &Channel) -> Result<()> {    channel.basic_qos(
        CONFIG.prefetch_count, 
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
        if let Err(err) = delivery {
            error!("Could not read from queue, {:?}", err);
            continue
        }

        let (channel, delivery) = delivery.unwrap();

        match JsonEmail::from_slice(delivery.data.as_slice()) {
            
            Ok(email) => {
                if let Err(err) = send_mail(email) {
                    error!("Could not dispatch email, {:?}", err);
                    continue
                };
            },

            Err(err) => {
                error!("Could not parse email, {:?}", err);
                continue
            }

        };
        
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