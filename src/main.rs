use axum::body::Body;
use axum::http::{Request, Response};
use axum::routing::any;
use axum::{routing::get, Router};
use std::collections::HashMap;
use std::env;
use std::net::SocketAddr;

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let port = env::var("PORT").unwrap_or_else(|_| "8080".to_string());
    let port = port.parse::<u16>().expect("PORT must be a number.");

    let app = Router::new()
        .route("/logo.svg", get(logo))
        .route("/", any(index));
        .route("/*path", any(index));
    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    println!("Listening on http://{}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn logo() -> Response<Body> {
    let body = include_bytes!("assets/jamsocket-logo.svg");
    let body = Body::from(body.to_vec());
    Response::builder()
        .header("content-type", "image/svg+xml")
        .body(body)
        .unwrap()
}

async fn index(request: Request<Body>) -> Response<Body> {
    let method = request.method();
    let path = request.uri().path();
    let query = request.uri().query().unwrap_or("");

    let request_str = format!(
        r#"
    <li>
        <samp class="key">method</samp>: <samp class="val">{method}</samp>
    </li>

    <li>
        <samp class="key">path</samp>: <samp class="val">{path}</samp>
    </li>

    <li>
        <samp class="key">query</samp>: <samp class="val">{query}</samp>
    </li>
    "#
    );

    let env: HashMap<String, String> = env::vars().collect();
    let env_str = env
        .iter()
        .filter(|(key, _)| key.starts_with("JAMSOCKET_") || key.as_str() == "PORT")
        .map(|(key, value)| {
            format!(
                r#"
                <li>
                    <samp class="key">{key}</samp>: <samp class="val">{value}</samp>
                </li>
            "#
            )
        })
        .collect::<Vec<String>>()
        .join("\n");

    let message = if env.contains_key("JAMSOCKET_URL") {
        "<h1>Hello from Jamsocket!</h1>"
    } else {
        r#"<p style="color: #ff0000;">It looks like we are not running on Jamsocket (<samp>JAMSOCKET_URL</samp> is not set).</p>"#
    };

    let html = format!(
        r#"
        <html>
            <head>
                <title>Jamsocket Hello World</title>
                <style>
                    body {{
                        color: white;
                        background: #1a1a1a;
                        font-family: Helvetica, sans-serif;
                        padding-top: 40px;
                    }}

                    body > div {{
                        max-width: 600px;
                        margin: auto;
                        overflow-wrap: break-word;
                    }}

                    body > div > div {{
                        display: flex;
                        flex-direction: column;
                        align-items: center;
                    }}

                    .key {{
                        color: #ff00ff;
                    }}

                    .val {{
                        color: #00ffff;
                    }}

                    ul {{
                        color: #999;
                    }}
                </style>
            </head>
            <body>
                <div>
                    <div>
                        <img src="/logo.svg" alt="Jamsocket Logo" style="width: 270px;" />
                        {message}
                    </div>
                    <p><strong>Environment variables:</strong></p>
                    <ul>
                        {env_str}
                    </ul>
                    <p><strong>Request:</strong></p>
                    <ul>
                        {request_str}
                    </ul>
                </div>
            </body>
        </html>
        "#
    );

    let body = Body::from(html);
    Response::builder()
        .header("content-type", "text/html")
        .body(body)
        .unwrap()
}
