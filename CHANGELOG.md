# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.1.1] - 2025-02-11

### 🔧 Changed
- Enhanced connector configurations for better reliability
- Updated Docker Compose configuration for improved stability
- Streamlined connector creation process with new shell script

### ✨ Added
- New `create-connectors.sh` script for automated connector deployment

## [0.1.0] - 2025-02-11

### Added
- Initial project setup with Docker Compose
- PostgreSQL source database setup
- PostgreSQL sink database setup
- Kafka and Zookeeper configuration
- Schema Registry integration
- Kafka Connect setup with JDBC connectors
- Automated connector installation and configuration
- Database initialization scripts with triggers for `updated_at`
- Source connector configuration with incrementing mode
- Sink connector configuration with upsert mode

### Changed
- Updated source connector to use `timestamp+incrementing` mode for capturing both new records and updates
- Fixed topic name mismatch between source and sink connectors
- Modified sink connector to read from the correct Kafka topic

### Fixed
- Resolved data synchronization issues by correcting topic name configuration
- Improved update detection by implementing timestamp-based tracking
- Ensured proper timestamp handling in both source and sink databases

### Technical Details
- Source database running on port 5432
- Sink database running on port 5433
- Kafka Connect REST API available on port 8083
- Schema Registry running on port 8081
- Kafka broker accessible on ports 9092 (external) and 29092 (internal)
- Implemented PostgreSQL triggers for automatic `updated_at` timestamp updates
- Configured JDBC connectors with proper error handling and schema management

### Dependencies
- PostgreSQL 15 (Alpine)
- Confluent Platform 7.4.1
  - Apache Kafka
  - Zookeeper
  - Schema Registry
  - Kafka Connect
- Confluent JDBC Connector 10.7.4