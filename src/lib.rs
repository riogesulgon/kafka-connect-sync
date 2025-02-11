use sqlx::postgres::PgPool;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow, Clone, PartialEq)]
pub struct Contact {
    pub id: Option<i32>,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub phone: Option<String>,
}

pub async fn create_test_pool() -> Result<PgPool, sqlx::Error> {
    let database_url = std::env::var("DATABASE_URL")
        .unwrap_or_else(|_| "postgres://contactuser:contactpassword@localhost:5432/contacts".to_string());
    
    PgPool::connect(&database_url).await
}

pub async fn create_contact(pool: &PgPool, contact: &Contact) -> Result<Contact, sqlx::Error> {
    let record = sqlx::query_as!(
        Contact,
        "INSERT INTO contacts (first_name, last_name, email, phone) 
         VALUES ($1, $2, $3, $4) 
         RETURNING id, first_name, last_name, email, phone",
        contact.first_name,
        contact.last_name,
        contact.email,
        contact.phone
    )
    .fetch_one(pool)
    .await?;

    Ok(record)
}

pub async fn get_contact_by_email(pool: &PgPool, email: &str) -> Result<Contact, sqlx::Error> {
    let contact = sqlx::query_as!(
        Contact,
        "SELECT id, first_name, last_name, email, phone FROM contacts WHERE email = $1",
        email
    )
    .fetch_one(pool)
    .await?;

    Ok(contact)
}

pub async fn list_contacts(pool: &PgPool) -> Result<Vec<Contact>, sqlx::Error> {
    let contacts = sqlx::query_as!(
        Contact,
        "SELECT id, first_name, last_name, email, phone FROM contacts"
    )
    .fetch_all(pool)
    .await?;

    Ok(contacts)
}

#[cfg(test)]
mod tests {
    use super::*;
    use sqlx::PgPool;

    #[tokio::test]
    async fn test_create_and_retrieve_contact() {
        let pool = create_test_pool().await.expect("Failed to create test pool");

        let new_contact = Contact {
            id: None,
            first_name: "Test".to_string(),
            last_name: "User".to_string(),
            email: "test.user@example.com".to_string(),
            phone: Some("+1-555-123-4567".to_string()),
        };

        // Create contact
        let created_contact = create_contact(&pool, &new_contact)
            .await
            .expect("Failed to create contact");

        assert!(created_contact.id.is_some());
        assert_eq!(created_contact.first_name, new_contact.first_name);
        assert_eq!(created_contact.email, new_contact.email);

        // Retrieve contact
        let retrieved_contact = get_contact_by_email(&pool, &new_contact.email)
            .await
            .expect("Failed to retrieve contact");

        assert_eq!(created_contact, retrieved_contact);
    }

    #[tokio::test]
    async fn test_list_contacts() {
        let pool = create_test_pool().await.expect("Failed to create test pool");

        // Ensure at least one contact exists
        let new_contact = Contact {
            id: None,
            first_name: "List".to_string(),
            last_name: "Test".to_string(),
            email: "list.test@example.com".to_string(),
            phone: Some("+1-555-987-6543".to_string()),
        };

        create_contact(&pool, &new_contact)
            .await
            .expect("Failed to create contact for list test");

        // List contacts
        let contacts = list_contacts(&pool)
            .await
            .expect("Failed to list contacts");

        assert!(!contacts.is_empty());
    }
}