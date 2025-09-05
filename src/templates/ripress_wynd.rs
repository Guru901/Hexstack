use ripress::{app::App, types::RouterFns};
use wynd::wynd::{WithRipress, Wynd};

#[tokio::main]
async fn main() {
    let mut wynd: Wynd<WithRipress> = Wynd::new();
    let mut app = App::new();

    wynd.on_connection(|conn| async move {
        conn.on_text(|event, handle| async move {
            let _ = handle.send_text(&format!("Echo: {}", event.data)).await;
        });
    });

    app.get("/", |_, res| async move { res.ok().text("Hello World!") });
    app.use_wynd("/ws", wynd.handler());

    app.listen(3000, || {
        println!("Server running on http://localhost:3000");
        println!("WebSocket available at ws://localhost:3000/ws");
    })
    .await;
}
