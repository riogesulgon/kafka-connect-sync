use kafka_connect::{Contact, create_test_pool, create_contact, get_contact_by_email, list_contacts};

use rdkafka::config::ClientConfig;
use rdkafka::producer::{FutureProducer, FutureRecord};
use std::error::Error;
use std::time::Duration;
use std::time::SystemTime;

async fn create_kafka_producer() -> Result<FutureProducer, Box<dyn Error>> {
    let producer: FutureProducer = ClientConfig::new()
        .set("bootstrap.servers", "0.0.0.0:9092")
        .set("client.id", "kafka-connect-producer")
        .set("acks", "all")  // Strongest acknowledgement
        .set("retries", "3")  // Retry on failure
        .set("retry.backoff.ms", "500")  // Backoff between retries
        .set("message.timeout.ms", "30000")
        .set("request.timeout.ms", "30000")
        .create()?;
    
    Ok(producer)
}

async fn publish_contact_to_kafka(producer: &FutureProducer, contact: &Contact) -> Result<(), Box<dyn Error>> {
    // Use serde_json to ensure proper JSON formatting
    let serialized_contact = serde_json::json!({
        "id": contact.id.unwrap_or(0),
        "first_name": contact.first_name,
        "last_name": contact.last_name,
        "email": contact.email,
        "phone": contact.phone
    }).to_string();
    
    let record = FutureRecord::to("contacts")
        .payload(&serialized_contact)
        .key(&contact.email);

    println!("Attempting to send message to Kafka...");
    println!("Topic: contacts");
    println!("Key: {}", contact.email);
    println!("Payload: {}", serialized_contact);

    println!("\nKafka connection details:");
    println!("Attempting to connect to broker at localhost:29092");
    
    match producer.send(record, Duration::from_secs(30)).await {
        Ok(delivery) => {
            println!("Message successfully sent!");
            println!("Delivery: {:?}", delivery);
            Ok(())
        },
        Err((kafka_error, message)) => {
            eprintln!("Kafka Error: {:?}", kafka_error);
            eprintln!("Failed Message: {:?}", message);
            Err(Box::new(kafka_error))
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Create database pool
    let pool = create_test_pool().await?;

    // Create Kafka producer
    let producer = create_kafka_producer().await?;

    // Generate a unique timestamp for email
    let timestamp = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)?
        .as_millis();

    // Example usage: Create a new contact
    let new_contact = Contact {
        id: None,
        first_name: "Jane".to_string(),
        last_name: "Smith".to_string(),
        email: format!("jane.smith.{}@example.com", timestamp),
        phone: Some("+1-555-987-6543".to_string()),
    };

    // Insert contact into database
    let created_contact = create_contact(&pool, &new_contact).await?;
    println!("Created contact: {:?}", created_contact);

    // Retrieve contact by email
    let retrieved_contact = get_contact_by_email(&pool, &created_contact.email).await?;
    println!("Retrieved contact: {:?}", retrieved_contact);

    // List all contacts
    let all_contacts = list_contacts(&pool).await?;
    println!("Total contacts: {}", all_contacts.len());

    // Publish contact to Kafka
    publish_contact_to_kafka(&producer, &created_contact).await?;
    println!("Published contact to Kafka");

    Ok(())
}
