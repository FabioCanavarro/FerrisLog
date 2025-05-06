#[derive(Clone, Copy)]
pub enum Engine {
    Kvs,
    Sled,
}

impl From<Engine> for String {
    fn from(value: Engine) -> Self {
        match value {
            Engine::Kvs => "Kvs".to_string(),
            Engine::Sled => "Sled".to_string(),
        }
    }
}

impl From<String> for Engine {
    fn from(value: String) -> Self {
        match value.to_lowercase().as_ref() {
            "kvs" => Engine::Kvs,
            "sled" => Engine::Sled,
            _ => panic!("Engine not chosen correctly"),
        }
    }
}
