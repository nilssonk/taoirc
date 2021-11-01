pub struct Message {
    pub prefix: Option<Prefix>,
    pub command: Command,
    pub params: Vec<String>,
}

pub struct UserPrefix {
    pub nick: String,
    pub user: Option<String>,
    pub host: Option<String>,
}

impl UserPrefix {
    pub fn from(n: String, u: Option<String>, h: Option<String>) -> Self {
        UserPrefix {
            nick: n,
            user: u,
            host: h,
        }
    }
}

pub enum Prefix {
    Server(String),
    User(UserPrefix),
}

impl Message {
    pub fn from(pr: Option<Prefix>, c: Command, pa: Vec<String>) -> Self {
        Message {
            prefix: pr,
            command: c,
            params: pa,
        }
    }
}

pub enum Command {
    String(String),
    Code(i16),
}
