/// 常量或配置

use std::time::{
    Duration,
};

pub const DB_URL: &str = "mysql://root:123456@localhost:3306"; // 数据库连接地址

pub const TOKEN_EXP_TIME: Duration = Duration::from_millis(2 * 60 * 60); // token 过期时间

pub const JWT_SECRET: &[u8; 6] = b"secret"; // jwt key

pub const STATIC_PATH: &str = "static"; // 静态文件路径

pub const NOT_AUTH_PATH: [&str; 5] = [ // 不需要鉴权的路径
    "/",
    "/login",
    "/logout",
    "/register",
    "/doc",
];