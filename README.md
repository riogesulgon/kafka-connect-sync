# Kafka Connect Sync System with Rust, PostgreSQL, and Kafka

## Project Overview
This project demonstrates a Kafka sync system using Rust, PostgreSQL, Kafka Connect, and Schema Registry. It provides a robust data synchronization solution with the following key components:

- PostgreSQL as the primary database
- Kafka for message streaming
- Kafka Connect for data integration
- Schema Registry for schema management
- Rust for application logic

## Prerequisites
- Docker
- Docker Compose
- Rust (latest stable version)
- Cargo
- curl (for registering Kafka Connect connectors)

## Setup and Installation

### 1. Start Infrastructure
```bash
docker-compose up -d
```

### 2. Install Rust Dependencies
```bash
cargo build
```

### 3. Register Kafka Connect Connector
```bash
curl -X POST -H "Content-Type: application/json" \
  --data @connectors/postgres-source-connector.json \
  http://localhost:8083/connectors
```

### 4. Run the Application
```bash
cargo run
```

## System Architecture

### Components
- **PostgreSQL**: Stores contact information
- **Kafka**: Message broker for data streaming
- **Kafka Connect**: Handles data integration
- **Schema Registry**: Manages and validates Avro schemas
- **Rust Application**: Manages contact data operations

### Data Flow
1. Contact data is written to PostgreSQL
2. Kafka Connect source connector captures changes
3. Changes are published to Kafka topics
4. Schema Registry validates message schemas

## Development

### Environment Variables
- `DATABASE_URL`: PostgreSQL connection string
- `KAFKA_BOOTSTRAP_SERVERS`: Kafka broker connection

### Testing
```bash
cargo test
```

## License
MIT License