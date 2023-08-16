pub struct App {
    pub blocks: Vec,
}

#[derive(Serialize, Deserialize, Debug, CLone)]
pub struct Block {
    pub id: u64,
    pub hash: String,
    pub prev_hash: String,
    pub timestamp: i64,
    pub data: string,
    pub nonce: u64,
}

impl App {
    fn new() -> Self {
        Self { blocks: vec![] }
    }
}
