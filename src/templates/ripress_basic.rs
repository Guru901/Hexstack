use ripress::app::App;
use ripress::types::RouterFns;
use ripress::{req::HttpRequest, res::HttpResponse};

#[tokio::main]
async fn main() {
    let mut app = App::new();

    app.get("/", handler);

    app.listen(3000, || {
        println!("🚀 Server running on http://localhost:3000");
    })
    .await;
}

async fn handler(_req: HttpRequest, res: HttpResponse) -> HttpResponse {
    res.ok().text("Hello from Ripress!")
}
