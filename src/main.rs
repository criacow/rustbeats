use lambda_http::{run, service_fn, Body, Error, Request, RequestExt, Response};
use std::time::SystemTime;

/// Main body. Calculate and return beats.
async fn function_handler(event: Request) -> Result<Response<Body>, Error> {
    let time = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH);
    let seconds = time?.as_secs();
    let beats = (((seconds + 3600) % 86400) * 10) / 864;

    let message = format!("<html><head><title>r u s t b e a t s  0.01</title></head><body><h1>The current time is: {:03}</h1></body></html>", beats);

    let resp = Response::builder()
        .status(200)
        .header("content-type", "text/html")
        .body(message.into())
        .map_err(Box::new)?;
    Ok(resp)
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        // disable printing the name of the module in every log line.
        .with_target(false)
        // disabling time is handy because CloudWatch will add the ingestion time.
        .without_time()
        .init();

    run(service_fn(function_handler)).await
}
