use axum::{extract::ws::WebSocketUpgrade, response::Html, routing::get, Router};
use kitsune_fe_native::app;

use crate::state::Zustand;

pub fn routes(_state: Zustand, port: u16) -> Router<Zustand> {
    let addr: std::net::SocketAddr = ([0, 0, 0, 0], port).into();

    let view = dioxus_liveview::LiveViewPool::new();
    let index_page_with_glue = |glue: &str| {
        Html(format!(
            r#"
        <!DOCTYPE html>
        <html>
            <head> <title>Dioxus LiveView with axum</title>  </head>
            <body> <div id="main"></div> </body>
            {glue}
        </html>
        "#,
        ))
    };

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
                        _ = view.launch(dioxus_liveview::axum_socket(socket), app).await;
                    })
                }),
            )
}
