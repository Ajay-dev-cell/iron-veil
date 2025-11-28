#!/bin/bash
set -e

# Colors for output
GREEN='\033[0;32m'
RED='\033[0;31m'
NC='\033[0m' # No Color

echo -e "${GREEN}Starting E2E Test Setup...${NC}"

# 1. Start Postgres Container
echo "Starting Postgres container..."
docker rm -f pg-test 2>/dev/null || true
docker run --name pg-test -e POSTGRES_PASSWORD=password -p 5432:5432 -d postgres > /dev/null

# Wait for Postgres to be ready
echo "Waiting for Postgres to be ready..."
sleep 3

# 2. Seed Data
echo "Seeding database..."
docker exec -i pg-test psql -U postgres -c "
DROP TABLE IF EXISTS users;
CREATE TABLE users (id SERIAL PRIMARY KEY, email TEXT, phone_number TEXT, address TEXT);
INSERT INTO users (email, phone_number, address) VALUES ('real.human@gmail.com', '555-0199', '123 Main St, New York, NY');
" > /dev/null

# 3. Run Query via Proxy
echo -e "${GREEN}Running Query via Proxy (port 6543)...${NC}"
echo "Expected: Masked data (e.g., fake email)"
echo "----------------------------------------"

# Use host.docker.internal for macOS compatibility
docker run --rm -i -e PGPASSWORD=password postgres psql -h host.docker.internal -p 6543 -U postgres -c "SELECT * FROM users;"

echo "----------------------------------------"
echo -e "${GREEN}Test Complete!${NC}"
echo "If you saw masked data above, the proxy is working."
