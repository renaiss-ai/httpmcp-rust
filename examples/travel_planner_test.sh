#!/bin/bash

# Travel Planner MCP Server - Test Commands
# Run the server first: cargo run --example travel_planner

BASE_URL="http://127.0.0.1:3001/mcp"

echo "========================================="
echo "Travel Planner MCP Server - Test Suite"
echo "========================================="
echo ""

# 1. Initialize
echo "1️⃣  Initialize connection..."
curl -s -X POST $BASE_URL \
  -H "Content-Type: application/json" \
  -d '{
    "jsonrpc": "2.0",
    "id": 1,
    "method": "initialize",
    "params": {
      "protocolVersion": "2024-11-05",
      "capabilities": {},
      "clientInfo": {"name": "test-client", "version": "1.0"}
    }
  }' | jq .
echo ""

# 2. List resources
echo "2️⃣  List travel resources..."
curl -s -X POST $BASE_URL \
  -H "Content-Type: application/json" \
  -H "x-user-id: user123" \
  -d '{
    "jsonrpc": "2.0",
    "id": 2,
    "method": "resources/list"
  }' | jq .
echo ""

# 3. Read popular destinations
echo "3️⃣  Get popular destinations..."
curl -s -X POST $BASE_URL \
  -H "Content-Type: application/json" \
  -d '{
    "jsonrpc": "2.0",
    "id": 3,
    "method": "resources/read",
    "params": {
      "uri": "travel://destinations/popular"
    }
  }' | jq .
echo ""

# 4. List tools
echo "4️⃣  List available tools..."
curl -s -X POST $BASE_URL \
  -H "Content-Type: application/json" \
  -d '{
    "jsonrpc": "2.0",
    "id": 4,
    "method": "tools/list"
  }' | jq .
echo ""

# 5. Search flights
echo "5️⃣  Search flights NYC → Paris..."
curl -s -X POST $BASE_URL \
  -H "Content-Type: application/json" \
  -d '{
    "jsonrpc": "2.0",
    "id": 5,
    "method": "tools/call",
    "params": {
      "name": "search_flights",
      "arguments": {
        "from": "NYC",
        "to": "Paris",
        "date": "2025-06-15",
        "passengers": 2
      }
    }
  }' | jq .
echo ""

# 6. Get weather
echo "6️⃣  Get weather for Paris..."
curl -s -X POST $BASE_URL \
  -H "Content-Type: application/json" \
  -d '{
    "jsonrpc": "2.0",
    "id": 6,
    "method": "tools/call",
    "params": {
      "name": "get_weather",
      "arguments": {
        "city": "Paris"
      }
    }
  }' | jq .
echo ""

# 7. Calculate budget
echo "7️⃣  Calculate trip budget..."
curl -s -X POST $BASE_URL \
  -H "Content-Type: application/json" \
  -d '{
    "jsonrpc": "2.0",
    "id": 7,
    "method": "tools/call",
    "params": {
      "name": "calculate_budget",
      "arguments": {
        "destination": "Paris",
        "duration": 7,
        "travelers": 2,
        "budget_type": "moderate"
      }
    }
  }' | jq .
echo ""

# 8. Convert currency
echo "8️⃣  Convert USD to EUR..."
curl -s -X POST $BASE_URL \
  -H "Content-Type: application/json" \
  -d '{
    "jsonrpc": "2.0",
    "id": 8,
    "method": "tools/call",
    "params": {
      "name": "convert_currency",
      "arguments": {
        "amount": 1000,
        "from": "USD",
        "to": "EUR"
      }
    }
  }' | jq .
echo ""

# 9. List prompts
echo "9️⃣  List available prompts..."
curl -s -X POST $BASE_URL \
  -H "Content-Type: application/json" \
  -d '{
    "jsonrpc": "2.0",
    "id": 9,
    "method": "prompts/list"
  }' | jq .
echo ""

# 10. Get trip planning prompt
echo "🔟 Get trip planning prompt..."
curl -s -X POST $BASE_URL \
  -H "Content-Type: application/json" \
  -d '{
    "jsonrpc": "2.0",
    "id": 10,
    "method": "prompts/get",
    "params": {
      "name": "plan_trip",
      "arguments": {
        "destination": "Tokyo",
        "duration": "10",
        "interests": "food, culture, technology"
      }
    }
  }' | jq .
echo ""

echo "========================================="
echo "✅ Test suite complete!"
echo "========================================="
