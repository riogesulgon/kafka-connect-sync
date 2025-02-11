#!/bin/bash

echo "Creating Source Connector..."
curl -X POST http://localhost:8083/connectors \
  -H "Content-Type: application/json" \
  -d @connectors/postgres-source-connector.json

echo -e "\nCreating Sink Connector..."
curl -X POST http://localhost:8083/connectors \
  -H "Content-Type: application/json" \
  -d @connectors/postgres-sink-connector.json

echo -e "\nConnector Status:"
curl -s http://localhost:8083/connectors | jq '.' 