use axum::{extract::ws::WebSocketUpgrade, response::Html, routing::get, Router};
use kitsune_config::{ServerConfiguration, UrlConfiguration};
use kitsune_fe_native::{app, AppProps};

use crate::state::Zustand;

pub fn routes(
    _state: Zustand,
    server_config: &ServerConfiguration,
    url_config: &UrlConfiguration,
) -> Router<Zustand> {
    let addr: std::net::SocketAddr = ([0, 0, 0, 0], server_config.port).into();

    let view = dioxus_liveview::LiveViewPool::new();
    let index_page_with_glue = |glue: &str| {
        Html(format!(
            r#"
        <!DOCTYPE html>
        <html>
            <head>
                <title>Dioxus LiveView with axum</title>
                <link rel="stylesheet" href="/public/style.css">
            </head>
            <body> <div id="main"></div> </body>
            {glue}
        </html>
        "#,
        ))
    };

    let default_host = Some(format!("{}://{}", url_config.scheme, url_config.domain));
    let props = AppProps { default_host };

    Router::new()
            .route(
                "/",
                get(move || async move {
                    index_page_with_glue(&dioxus_liveview::interpreter_glue(&format!(
                        "ws://{addr}/experimental-fe/ws"
                    )))
                }),
            )
            .route(
                "/as-path",
                get(move || async move {
                    index_page_with_glue(&dioxus_liveview::interpreter_glue("/ws"))
                }),
            )
            .route(
                "/ws",
                get(move |ws: WebSocketUpgrade| async move {
                    ws.on_upgrade(move |socket| async move {
                        _ = view.launch_with_props(dioxus_liveview::axum_socket(socket), app, props).await;
                    })
                }),
            )
}
