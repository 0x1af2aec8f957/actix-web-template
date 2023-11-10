/// 常用方法

use serde::{Serialize, Deserialize};
use jsonwebtoken::{encode, decode, Header, Algorithm, Validation, EncodingKey, DecodingKey, TokenData};

use crate::utils::{constant};

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims { // jwt 的 payload
    aud: String, // 接收jwt的一方，可以使用 agent 表示
    iss: String, // 签发者
    exp: u64, // 过期时间
    user_id: String, // 用户ID
    sub: String, // 额外信息
}

impl Claims {
    fn default() -> Self { // 转换为 sub
        Self::from_user_id(String::from(""))
    }

    fn from_user_id(user_id: String) -> Self { // 从 sub 生成
        Self::from_agent(None, user_id)
    }

    fn from_agent(agent: Option<String>, user_id: String) -> Self { // 从 agent 和 sub 生成
        Self {
            user_id,
            aud: agent.unwrap_or(env!("CARGO_PKG_NAME").to_owned()),
            iss: env!("CARGO_PKG_NAME").to_owned(),
            exp: constant::TOKEN_EXP_TIME.as_secs(), // 过期时间
            sub: "example".to_owned(), // 跟当前 token 的用户信息相关
        }
    }
}

pub fn encode_token(user_id: String) -> Result<String, jsonwebtoken::errors::Error> { // token 编码
    let claims = Claims::from_user_id(user_id);

    encode(&Header::default(), &claims, &EncodingKey::from_secret(constant::JWT_SECRET.as_ref()))
}

pub fn decode_token(token: String, validation: Option<&Validation>) -> Result<TokenData<Claims>, jsonwebtoken::errors::Error> { // token 解码
    let binding = Validation::default();
    let _validation = validation.unwrap_or(&binding);
    decode::<Claims>(&token, &DecodingKey::from_secret(constant::JWT_SECRET.as_ref()), _validation)
}

pub fn validation_token(token: String) -> bool { // token 验证
    let claims = Claims::default();

    let mut validation = Validation::new(Algorithm::HS256);

    validation.set_audience(&[claims.aud]);
    validation.set_issuer(&[claims.iss]);

    validation.sub = Some(claims.sub);
    decode_token(token, Some(&validation)).is_ok()
}