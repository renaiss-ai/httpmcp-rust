use actix_web::web::Bytes;
use serde::Serialize;
use uuid::Uuid;

/// Server-Sent Event
#[derive(Debug, Clone)]
pub struct SseEvent {
    pub id: Option<String>,
    pub event: Option<String>,
    pub data: String,
}

impl SseEvent {
    /// Create a new SSE event
    pub fn new(data: impl Into<String>) -> Self {
        Self {
            id: Some(Uuid::new_v4().to_string()),
            event: Some("message".to_string()),
            data: data.into(),
        }
    }

    /// Create event with custom event type
    pub fn with_event(mut self, event: impl Into<String>) -> Self {
        self.event = Some(event.into());
        self
    }

    /// Create event from JSON-serializable data
    pub fn from_json<T: Serialize>(data: &T) -> Result<Self, serde_json::Error> {
        let json = serde_json::to_string(data)?;
        Ok(Self::new(json))
    }

    /// Convert to SSE format bytes
    pub fn to_bytes(&self) -> Bytes {
        let mut output = String::new();

        if let Some(id) = &self.id {
            output.push_str(&format!("id: {}\n", id));
        }

        if let Some(event) = &self.event {
            output.push_str(&format!("event: {}\n", event));
        }

        output.push_str(&format!("data: {}\n\n", self.data));

        Bytes::from(output)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sse_event_format() {
        let event = SseEvent {
            id: Some("123".to_string()),
            event: Some("message".to_string()),
            data: "test data".to_string(),
        };

        let bytes = event.to_bytes();
        let text = String::from_utf8(bytes.to_vec()).unwrap();

        assert!(text.contains("id: 123\n"));
        assert!(text.contains("event: message\n"));
        assert!(text.contains("data: test data\n"));
    }
}
