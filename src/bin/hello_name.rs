use lambda_http::{service_fn, Error, IntoResponse, Request, RequestExt};

#[tokio::main]
async fn main() -> Result<(), Error> {
    lambda_http::run(service_fn(hello)).await?;
    Ok(())
}

async fn hello( request: Request) -> Result<impl IntoResponse, Error> {
    let context = request.lambda_context();
    let query = request.query_string_parameters();
    let name = query.first("name").ok_or("stranger")?;

    dbg!(context);
    dbg!(name);
    Ok(format!(
        "wow nice hello {name} cool one",
    ))
}
