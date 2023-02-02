use actix_web::{App, HttpServer};
mod views;
use views::views_factory;
mod process;
mod state;
mod to_do;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        let app = App::new().configure(views_factory);

        println!("App is running at http://localhost:8080");
        return app;
    })
    // 手动指定启动的工作者数，默认是和 CPU 核数一样
    // .workers(4)
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
