use crate::protocol::*;
use serde_json::json;

/// Metadata builder for tools
#[derive(Debug, Clone)]
pub struct ToolMeta {
    description: Option<String>,
    params: Vec<ParamMeta>,
    required: Vec<String>,
}

#[derive(Debug, Clone)]
struct ParamMeta {
    name: String,
    param_type: String,
    description: String,
}

impl ToolMeta {
    pub fn new() -> Self {
        Self {
            description: None,
            params: Vec::new(),
            required: Vec::new(),
        }
    }

    pub fn description(mut self, desc: impl Into<String>) -> Self {
        self.description = Some(desc.into());
        self
    }

    pub fn param(
        mut self,
        name: impl Into<String>,
        param_type: impl Into<String>,
        description: impl Into<String>,
    ) -> Self {
        self.params.push(ParamMeta {
            name: name.into(),
            param_type: param_type.into(),
            description: description.into(),
        });
        self
    }

    pub fn required(mut self, fields: &[&str]) -> Self {
        self.required = fields.iter().map(|s| s.to_string()).collect();
        self
    }

    pub fn to_tool(&self, name: impl Into<String>) -> Tool {
        let mut properties = serde_json::Map::new();

        for param in &self.params {
            properties.insert(
                param.name.clone(),
                json!({
                    "type": param.param_type,
                    "description": param.description
                }),
            );
        }

        let mut schema = json!({
            "type": "object",
            "properties": properties
        });

        if !self.required.is_empty() {
            schema["required"] = json!(self.required);
        }

        Tool {
            name: name.into(),
            description: self.description.clone(),
            input_schema: schema,
        }
    }
}

impl Default for ToolMeta {
    fn default() -> Self {
        Self::new()
    }
}

/// Metadata builder for resources
#[derive(Debug, Clone)]
pub struct ResourceMeta {
    name: String,
    description: Option<String>,
    mime_type: Option<String>,
}

impl ResourceMeta {
    pub fn new() -> Self {
        Self {
            name: String::new(),
            description: None,
            mime_type: None,
        }
    }

    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.name = name.into();
        self
    }

    pub fn description(mut self, desc: impl Into<String>) -> Self {
        self.description = Some(desc.into());
        self
    }

    pub fn mime_type(mut self, mime: impl Into<String>) -> Self {
        self.mime_type = Some(mime.into());
        self
    }

    pub fn to_resource(&self, uri: impl Into<String>) -> Resource {
        Resource {
            uri: uri.into(),
            name: self.name.clone(),
            description: self.description.clone(),
            mime_type: self.mime_type.clone(),
        }
    }
}

impl Default for ResourceMeta {
    fn default() -> Self {
        Self::new()
    }
}

/// Metadata builder for prompts
#[derive(Debug, Clone)]
pub struct PromptMeta {
    description: Option<String>,
    arguments: Vec<PromptArgumentMeta>,
}

#[derive(Debug, Clone)]
struct PromptArgumentMeta {
    name: String,
    description: Option<String>,
    required: bool,
}

impl PromptMeta {
    pub fn new() -> Self {
        Self {
            description: None,
            arguments: Vec::new(),
        }
    }

    pub fn description(mut self, desc: impl Into<String>) -> Self {
        self.description = Some(desc.into());
        self
    }

    pub fn arg(
        mut self,
        name: impl Into<String>,
        description: impl Into<String>,
        required: bool,
    ) -> Self {
        self.arguments.push(PromptArgumentMeta {
            name: name.into(),
            description: Some(description.into()),
            required,
        });
        self
    }

    pub fn to_prompt(&self, name: impl Into<String>) -> Prompt {
        let arguments = if self.arguments.is_empty() {
            None
        } else {
            Some(
                self.arguments
                    .iter()
                    .map(|arg| PromptArgument {
                        name: arg.name.clone(),
                        description: arg.description.clone(),
                        required: Some(arg.required),
                    })
                    .collect(),
            )
        };

        Prompt {
            name: name.into(),
            description: self.description.clone(),
            arguments,
        }
    }
}

impl Default for PromptMeta {
    fn default() -> Self {
        Self::new()
    }
}
