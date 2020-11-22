use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct Grammar {
    pub rshift: String,
    pub lshift: String,
    pub inc: String,
    pub dec: String,
    pub write: String,
    pub read: String,
    pub loop_begin: String,
    pub loop_end: String,
}
