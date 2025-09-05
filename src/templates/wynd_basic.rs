use wynd::wynd::{Standalone, Wynd};

#[tokio::main]
async fn main() {
    let mut wynd: Wynd<Standalone> = Wynd::new();

    wynd.on_connection(|conn| async move {
        conn.on_text(|event, handle| async move {
            handle
                .send_text(&format!("Echo: {}", event.data))
                .await
                .unwrap();
        });
    });

    wynd.listen(3000, || {
        println!("Server running on ws://localhost:3000");
    })
    .await
    .unwrap();
}
