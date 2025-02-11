#!/bin/bash

echo "=== Source Contacts Table (Port 5432) ==="
PGPASSWORD=contactpassword psql -h localhost -p 5432 -U contactuser -d contacts -c "SELECT * FROM contacts ORDER BY id;"

echo -e "\n=== Sink Contacts Table (Port 5433) ==="
PGPASSWORD=sinkpassword psql -h localhost -p 5433 -U sinkuser -d contacts_sink -c "SELECT * FROM contacts ORDER BY id;" 