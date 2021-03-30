use serde::{Serialize, Deserialize};

#[derive(Serialize, sqlx::FromRow)]
pub struct UserInfo {
    id: String,
    username: String,
    email: String,
    nickname: String,
    avatar: String,
}

impl UserInfo {
    pub fn id(&self) -> &str {
        self.id.as_str()
    }
}

#[derive(Deserialize)]
pub struct UserRegisterRequest {
    username: String,
    password: String,
    email: String,
    nickname: String,
    avatar: String,

    #[serde(rename = "2fa_secret")]
    secret_2fa: Option<String>,

    invite_code: Option<String>,
}

impl UserRegisterRequest {
    #[inline]
    pub fn username(&self) -> &str {
        self.username.as_str()
    }

    #[inline]
    pub fn password(&self) -> &str {
        self.password.as_str()
    }

    #[inline]
    pub fn email(&self) -> &str {
        self.email.as_str()
    }

    #[inline]
    pub fn nickname(&self) -> &str {
        self.nickname.as_str()
    }

    #[inline]
    pub fn avatar(&self) -> &str {
        self.avatar.as_str()
    }

    #[inline]
    pub fn secret_2fa(&self) -> Option<&str> {
        self.secret_2fa.as_deref()
    }

    #[inline]
    pub fn invite_code(&self) -> Option<&str> {
        self.invite_code.as_deref()
    }
}

#[derive(Deserialize)]
pub struct UserRegisterCheckRequest {
    email: Option<String>,
    username: Option<String>,
}

impl UserRegisterCheckRequest {
    #[inline]
    pub fn username(&self) -> Option<&str> {
        self.username.as_deref()
    }

    #[inline]
    pub fn email(&self) -> Option<&str> {
        self.email.as_deref()
    }
}