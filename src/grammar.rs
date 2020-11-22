use serde::Deserialize;
#[derive(Debug, Clone, Deserialize)]
pub struct Grammar {
    pub rshift: Vec<String>,
    pub lshift: Vec<String>,
    pub inc: Vec<String>,
    pub dec: Vec<String>,
    pub write: Vec<String>,
    pub read: Vec<String>,
    pub loop_begin: Vec<String>,
    pub loop_end: Vec<String>,
}
