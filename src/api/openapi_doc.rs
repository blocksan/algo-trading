use std::collections::HashMap;

pub struct ApiDocumentation {
    pub paths: HashMap<String, PathItem>,
    pub components: Components,
}

pub struct PathItem {
    pub get: Option<Operation>,
    pub post: Option<Operation>,
    pub put: Option<Operation>,
    pub delete: Option<Operation>,
    // other HTTP methods
}

pub struct Operation {
    pub summary: String,
    pub description: String,
    pub parameters: Vec<Parameter>,
    pub request_body: Option<RequestBody>,
    pub responses: HashMap<String, Response>,
    pub tags: Vec<String>,
}

pub struct Parameter {
    pub name: String,
    pub location: ParameterLocation,
    pub description: String,
    pub required: bool,
    pub schema: Schema,
}

pub enum ParameterLocation {
    Path,
    Query,
    Header,
    Cookie,
}

pub struct RequestBody {
    pub description: String,
    pub required: bool,
    pub content: HashMap<String, MediaType>,
}

pub struct MediaType {
    pub schema: Schema,
    pub example: Option<String>,
}

pub struct Response {
    pub description: String,
    pub content: HashMap<String, MediaType>,
}

pub struct Schema {
    pub type_name: String,
    pub format: Option<String>,
    pub properties: Option<HashMap<String, Schema>>,
    pub items: Option<Box<Schema>>,
    pub reference: Option<String>,
}

pub struct Components {
    pub schemas: HashMap<String, Schema>,
}

// Attribute macro for documenting API endpoints
#[macro_export]
macro_rules! api_doc {
    (
        $(#[$meta:meta])*
        $vis:vis fn $name:ident $args:tt $(-> $ret:ty)? $body:block
    ) => {
        $(#[$meta])*
        $vis fn $name $args $(-> $ret)? $body
    };
} 