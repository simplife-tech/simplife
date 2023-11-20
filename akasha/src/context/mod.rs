use std::collections::HashMap;

use opentelemetry::Context as opentelemetry_Context;

#[derive(Clone, Default)]
pub struct Context {
    pub opentelemetry_context: opentelemetry_Context,
    pub keys_i64: HashMap<&'static str, i64>
}

impl Context {
    pub fn new_from_tonic_request<T>(request: &tonic::Request<T>) -> Context {
        let extensions = request.extensions();
        let oc = extensions.get::<opentelemetry_Context>().unwrap();
        Context { 
            opentelemetry_context: oc.clone(),
            ..Default::default()
        }
    }
}