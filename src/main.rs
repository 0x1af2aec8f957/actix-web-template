/// doc: https://github.com/actix/examples

mod middleware;
mod utils;
mod db;
mod router;

use actix_files as fs;
use actix_web::{
    get,
    post,
    web,
    App,
    HttpResponse,
    HttpServer,
    Responder,
    middleware::{
        Compress,
        Logger,
        DefaultHeaders,
    }
};

/***
    #[get("/")]
    async fn hello() -> impl Responder {
        HttpResponse::Ok().body("Hello world!")
    }

    /// extract `Info` using serde
    #[get("/")]
    async fn index(info: web::Json<Info>) -> Result<String> {
        Ok(format!("Welcome {}!", info.username))
    }
***/

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let db = db::Uri::new(Some(String::from("0.0.0.0")), None).connect().await; // 连接数据库

    /***
        如果需要使用路径区分业务请参考 `url-dispatch`: https://actix.rs/docs/url-dispatch#scoping-routes
        例子:
            HttpServer::new(|| {
                App::new()
                    .service(
                        web::scope("/admin")
                            .guard(guard::Host("admin.rust-lang.org"))
                            .route("", web::to(|| async { HttpResponse::Ok().body("admin") })),
                    )
                    .service(
                        web::scope("/web")
                            .guard(guard::Host("web.rust-lang.org"))
                            .route("", web::to(|| async { HttpResponse::Ok().body("web") })),
                    )
                    .route("/", web::to(HttpResponse::Ok))
            })
    ***/

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(db.clone())) // 数据库
            .wrap(DefaultHeaders::new().add(("X-Version", env!("CARGO_PKG_VERSION")))) // 头部信息
            .wrap(Compress::default()) // 根据请求的Accept-Encoding标头执行自动内容压缩协商
            .wrap(Logger::default()) // 日志
            .service(fs::Files::new("/static", ".").show_files_listing()) // 静态文件
            .route("/", web::get().to( || async { Some("welcome to actix_web!") })) // 欢迎词
            // 用户
            .service(router::user::add_user)
            .service(router::user::get_user)
    })
    .bind(("127.0.0.1", 3000))?
    .run()
    .await
}