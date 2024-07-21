use rabbitmq_stream_client::Environment;
use tracing::warn;

pub async fn init_broker() -> Environment {
    let mut count = 0;
    loop {
        if count == 100 {
            panic!("Hit broker connect timeout!")
        }

        let environment = Environment::builder()
            .host("127.0.0.1")
            .port(5552)
            .build()
            .await;

        if let Err(e) = environment {
            warn!("Cannot connect to broker, retrying {:?}", e);
        } else {
            return environment.unwrap();
        }

        count += 1;
    }
}
