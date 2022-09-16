use lambda_http::{run, service_fn, Body, Error, Request, Response};
use svg::node::element::path::Data;
use svg::node::element::Path;
use svg::Document;
use tracing::info;

fn create_svg() -> Document {
    let data = Data::new()
        .move_to((10, 10))
        .line_by((0, 50))
        .line_by((50, 0))
        .line_by((0, -50))
        .close();

    let path = Path::new()
        .set("fill", "none")
        .set("stroke", "black")
        .set("stroke-width", 3)
        .set("d", data);

    Document::new().set("viewBox", (0, 0, 70, 70)).add(path)
}

async fn function_handler(_event: Request) -> Result<Response<Body>, Error> {
    let svg_doc = create_svg();
    let mut svg_data: Vec<u8> = Vec::new();

    svg::write(&mut svg_data, &svg_doc)?;

    let resp = Response::builder()
        .status(200)
        .header("content-type", "image/svg+xml")
        .body(svg_data.into())
        .map_err(Box::new)?;

    info!("{resp:?}");

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
