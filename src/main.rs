use lambda_http::{run, service_fn, tracing, Body, Error, Request, RequestExt, Response};

/// This is the main body for the function.
/// Write your code inside it.
/// There are some code example in the following URLs:
/// - https://github.com/awslabs/aws-lambda-rust-runtime/tree/main/examples
async fn function_handler(event: Request) -> Result<Response<Body>, Error> {
    // Extract some useful information from the request
    let who = event
        .query_string_parameters_ref()
        .and_then(|params| params.first("name"))
        .unwrap_or("world");
    let message = format!("Hello {who}, from the Rust API");

    // Return something that implements IntoResponse.
    // It will be serialized to the right response event automatically by the runtime
    let resp = Response::builder()
        .status(200)
        .header("content-type", "text/html")
        .body(message.into())
        .map_err(Box::new)?;
    Ok(resp)
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing::init_default_subscriber();

    run(service_fn(function_handler)).await
}

#[cfg(test)]
mod tests {
    use super::*;
    use lambda_http::http::Method;
    use lambda_http::{Body as LambdaBody, Request};
    use std::collections::HashMap;

    #[tokio::test]
    async fn test_function_handler_default() {
        // Create a request with no query parameters
        let request = Request::new(LambdaBody::Empty);
        
        // Call the handler
        let response = function_handler(request).await.expect("Expected Ok response");
        
        // Convert the body to a string
        let body = response.body();
        let body_str = match body {
            LambdaBody::Text(s) => s,
            _ => panic!("Expected Text body"),
        };
        
        // Check that the response contains the expected message
        assert_eq!(body_str, "Hello world, from the Rust API");
    }

    #[tokio::test]
    async fn test_function_handler_with_name() {
        // Create a request with a query parameter
        let mut request = Request::new(LambdaBody::Empty);
        *request.method_mut() = Method::GET;
        
        // Add query parameters
        let mut query_params = HashMap::new();
        query_params.insert("name".to_string(), vec!["Rust".to_string()]);
        request.set_query_string_parameters(query_params);
        
        // Call the handler
        let response = function_handler(request).await.expect("Expected Ok response");
        
        // Convert the body to a string
        let body = response.body();
        let body_str = match body {
            LambdaBody::Text(s) => s,
            _ => panic!("Expected Text body"),
        };
        
        // Check that the response contains the expected message with the name parameter
        assert_eq!(body_str, "Hello Rust, from the Rust API");
    }
}
