use httpmcp_rust::protocol::*;
use httpmcp_rust::{HttpMcpServer, PromptMeta, RequestContext, ResourceMeta, Result, ToolMeta};
use serde_json::{json, Value};
use std::collections::HashMap;

// Resource Handlers
async fn list_destinations(
    _cursor: Option<String>,
    ctx: RequestContext,
) -> Result<(Vec<Resource>, Option<String>)> {
    let user = ctx.get_custom_header("x-user-id").unwrap_or_default();
    tracing::info!("Listing travel resources for user: {}", user);

    Ok((
        vec![
            Resource {
                uri: "travel://destinations/popular".to_string(),
                name: "Popular Destinations".to_string(),
                description: Some("Popular travel destinations".to_string()),
                mime_type: Some("application/json".to_string()),
            },
            Resource {
                uri: "travel://guides/tips".to_string(),
                name: "Travel Tips".to_string(),
                description: Some("General travel advice".to_string()),
                mime_type: Some("text/markdown".to_string()),
            },
        ],
        None,
    ))
}

async fn read_destination(uri: String, _ctx: RequestContext) -> Result<Vec<ResourceContents>> {
    let content = match uri.as_str() {
        "travel://destinations/popular" => ResourceContents {
            uri,
            mime_type: Some("application/json".to_string()),
            text: Some(json!({
                "destinations": [
                    {"id": "paris", "name": "Paris", "category": "city", "best_season": "Spring"},
                    {"id": "tokyo", "name": "Tokyo", "category": "city", "best_season": "Spring"},
                    {"id": "bali", "name": "Bali", "category": "beach", "best_season": "May-Sep"}
                ]
            }).to_string()),
            blob: None,
        },
        "travel://guides/tips" => ResourceContents {
            uri,
            mime_type: Some("text/markdown".to_string()),
            text: Some("# Travel Tips\n\n- Check passport\n- Get insurance\n- Pack light".to_string()),
            blob: None,
        },
        _ => return Err(httpmcp_rust::McpError::ResourceNotFound(uri)),
    };
    Ok(vec![content])
}

// Tool Handlers
async fn search_flights(args: HashMap<String, Value>, _ctx: RequestContext) -> Result<Value> {
    let from = args.get("from").and_then(|v| v.as_str()).unwrap_or("NYC");
    let to = args.get("to").and_then(|v| v.as_str()).unwrap_or("Paris");
    let date = args
        .get("date")
        .and_then(|v| v.as_str())
        .unwrap_or("2025-06-15");

    Ok(json!({
        "flights": [
            {
                "airline": "Air France",
                "from": from,
                "to": to,
                "departure": format!("{}T08:00", date),
                "price": "$850",
                "duration": "8h 30m"
            }
        ]
    }))
}

async fn search_hotels(args: HashMap<String, Value>, _ctx: RequestContext) -> Result<Value> {
    let city = args.get("city").and_then(|v| v.as_str()).unwrap_or("Paris");

    Ok(json!({
        "hotels": [
            {
                "name": "Grand Hotel",
                "city": city,
                "rating": 4.5,
                "price_per_night": "$180"
            }
        ]
    }))
}

async fn get_weather(args: HashMap<String, Value>, _ctx: RequestContext) -> Result<Value> {
    let city = args.get("city").and_then(|v| v.as_str()).unwrap_or("Paris");

    Ok(json!({
        "city": city,
        "temperature": "22°C",
        "condition": "Partly Cloudy"
    }))
}

async fn calculate_budget(args: HashMap<String, Value>, _ctx: RequestContext) -> Result<Value> {
    let destination = args
        .get("destination")
        .and_then(|v| v.as_str())
        .unwrap_or("Paris");
    let duration = args.get("duration").and_then(|v| v.as_f64()).unwrap_or(7.0);
    let travelers = args
        .get("travelers")
        .and_then(|v| v.as_f64())
        .unwrap_or(1.0);

    let daily_rate = 150.0;
    let total = (daily_rate * duration + 200.0) * travelers;

    Ok(json!({
        "destination": destination,
        "duration": format!("{} days", duration),
        "total": format!("${:.2}", total)
    }))
}

async fn convert_currency(args: HashMap<String, Value>, _ctx: RequestContext) -> Result<Value> {
    let amount = args.get("amount").and_then(|v| v.as_f64()).unwrap_or(100.0);
    let from = args.get("from").and_then(|v| v.as_str()).unwrap_or("USD");
    let to = args.get("to").and_then(|v| v.as_str()).unwrap_or("EUR");

    let rate = if from == "USD" && to == "EUR" {
        0.92
    } else {
        1.0
    };
    let converted = amount * rate;

    Ok(json!({
        "from": {"currency": from, "amount": amount},
        "to": {"currency": to, "amount": converted},
        "rate": rate
    }))
}

// Prompt Handlers
async fn plan_trip_prompt(
    _name: String,
    args: Option<HashMap<String, String>>,
    _ctx: RequestContext,
) -> Result<(Option<String>, Vec<PromptMessage>)> {
    let dest = args
        .as_ref()
        .and_then(|a| a.get("destination"))
        .cloned()
        .unwrap_or_default();
    let duration = args
        .as_ref()
        .and_then(|a| a.get("duration"))
        .cloned()
        .unwrap_or_default();
    let interests = args
        .as_ref()
        .and_then(|a| a.get("interests"))
        .cloned()
        .unwrap_or("general".to_string());

    Ok((Some("Trip planning".to_string()), vec![PromptMessage {
        role: "user".to_string(),
        content: PromptContent::Text {
            text: format!(
                "Plan a {} day trip to {}. Interests: {}. Include activities, restaurants, and tips.",
                duration, dest, interests
            ),
        },
    }]))
}

async fn budget_advice_prompt(
    _name: String,
    args: Option<HashMap<String, String>>,
    _ctx: RequestContext,
) -> Result<(Option<String>, Vec<PromptMessage>)> {
    let dest = args
        .as_ref()
        .and_then(|a| a.get("destination"))
        .cloned()
        .unwrap_or_default();
    let budget = args
        .as_ref()
        .and_then(|a| a.get("budget"))
        .cloned()
        .unwrap_or_default();

    Ok((Some("Budget planning".to_string()), vec![PromptMessage {
        role: "user".to_string(),
        content: PromptContent::Text {
            text: format!(
                "I have ${} for {}. Help me budget for accommodation, food, activities, and transport.",
                budget, dest
            ),
        },
    }]))
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter("info,httpmcp_rust=debug")
        .init();

    let server = HttpMcpServer::builder()
        .name("travel-planner-mcp")
        .version("1.0.0")
        // Resources
        .resource(
            "travel://destinations/popular",
            ResourceMeta::new()
                .name("Destinations")
                .mime_type("application/json"),
            list_destinations,
            read_destination,
        )
        // Tools
        .tool(
            "search_flights",
            ToolMeta::new()
                .description("Search flights")
                .param("from", "string", "Departure city")
                .param("to", "string", "Destination")
                .param("date", "string", "Date (YYYY-MM-DD)")
                .required(&["from", "to", "date"]),
            search_flights,
        )
        .tool(
            "search_hotels",
            ToolMeta::new()
                .description("Search hotels")
                .param("city", "string", "City name")
                .param("checkin", "string", "Check-in date")
                .param("checkout", "string", "Check-out date")
                .required(&["city", "checkin", "checkout"]),
            search_hotels,
        )
        .tool(
            "get_weather",
            ToolMeta::new()
                .description("Get weather")
                .param("city", "string", "City name")
                .required(&["city"]),
            get_weather,
        )
        .tool(
            "calculate_budget",
            ToolMeta::new()
                .description("Calculate trip budget")
                .param("destination", "string", "Destination")
                .param("duration", "number", "Days")
                .param("travelers", "number", "Number of travelers")
                .required(&["destination", "duration", "travelers"]),
            calculate_budget,
        )
        .tool(
            "convert_currency",
            ToolMeta::new()
                .description("Convert currency")
                .param("amount", "number", "Amount")
                .param("from", "string", "From currency")
                .param("to", "string", "To currency")
                .required(&["amount", "from", "to"]),
            convert_currency,
        )
        // Prompts
        .prompt(
            "plan_trip",
            PromptMeta::new()
                .description("Plan a complete trip")
                .arg("destination", "Destination", true)
                .arg("duration", "Duration in days", true)
                .arg("interests", "Travel interests", false),
            plan_trip_prompt,
        )
        .prompt(
            "budget_advice",
            PromptMeta::new()
                .description("Budget recommendations")
                .arg("destination", "Destination", true)
                .arg("budget", "Total budget", true),
            budget_advice_prompt,
        )
        .enable_cors(true)
        .build()
        .expect("Failed to build server");

    println!("✈️  Travel Planner MCP Server");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("🌐 http://127.0.0.1:3001/mcp");
    println!();
    println!("📚 Resources: destinations, guides");
    println!("🔧 Tools: flights, hotels, weather, budget, currency");
    println!("💡 Prompts: plan_trip, budget_advice");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");

    server.run("127.0.0.1:3001").await
}
