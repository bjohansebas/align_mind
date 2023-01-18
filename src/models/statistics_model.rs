use rocket::serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct AllEmotions {
    pub positive: Vec<(String, u32, String)>,
    pub negative: Vec<(String, u32, String)>,
}
