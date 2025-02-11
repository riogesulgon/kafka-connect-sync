# 🔄 Kafka Connect Sync System with Rust, PostgreSQL, and Kafka

## 🎯 Project Overview
This project demonstrates a Kafka sync system using Rust, PostgreSQL, Kafka Connect, and Schema Registry. It provides a robust data synchronization solution with the following key components:

- 🐘 PostgreSQL as the primary database
- 🚀 Kafka for message streaming
- 🔌 Kafka Connect for data integration
- 📋 Schema Registry for schema management
- 🦀 Rust for application logic

## 📋 Prerequisites
- 🐳 Docker
- 🐳 Docker Compose
- 🦀 Rust (latest stable version)
- 📦 Cargo
- 🔄 curl (for registering Kafka Connect connectors)
- 🐘 PostgreSQL client (psql)

## 🚀 Setup and Installation

### 1. Start Infrastructure
```bash
docker-compose up -d
```

### 2. Create Kafka Connect Connectors
The project includes an automated script to create and configure both source and sink connectors:

```bash
./create-connectors.sh
```

This script will:
- 🔄 Create the source connector to capture changes from the source PostgreSQL database
- 💾 Create the sink connector to write data to the target PostgreSQL database
- 📊 Display the status of all connectors
- ✅ Verify connector configurations and health

You can verify the connectors are running by checking:
```bash
curl -s http://localhost:8083/connectors | jq '.'
```

### 3. Install Rust Dependencies
```bash
cargo build
```

### 4. Run the Application
```bash
cargo run
```

### 5. View Contact Tables
To view and compare the contents of both source and sink contact tables, use the provided script:

```bash
./scripts/view_contacts.sh
```

This script will display:
- 📤 Source contacts table (from the database on port 5432)
- 📥 Sink contacts table (from the database on port 5433)

Make sure the Docker containers are running before executing the script.

## 🏗️ System Architecture

### Components
- 🐘 **PostgreSQL**: Stores contact information
- 🚀 **Kafka**: Message broker for data streaming
- 🔌 **Kafka Connect**: Handles data integration
- 📋 **Schema Registry**: Manages and validates Avro schemas
- 🦀 **Rust Application**: Manages contact data operations

### Data Flow
1. ✍️ Contact data is written to PostgreSQL
2. 🔄 Kafka Connect source connector captures changes
3. 📨 Changes are published to Kafka topics
4. ✅ Schema Registry validates message schemas

## 👩‍💻 Development

### Environment Variables
- 🔑 `DATABASE_URL`: PostgreSQL connection string
- 🔌 `KAFKA_BOOTSTRAP_SERVERS`: Kafka broker connection

### Testing
```bash
cargo test
```

## 📄 License
MIT License