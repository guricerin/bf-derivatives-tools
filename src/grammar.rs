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

// impl Grammar {
//     pub fn new(
//         rshift: &str,
//         lshift: &str,
//         inc: &str,
//         dec: &str,
//         write: &str,
//         read: &str,
//         loop_begin: &str,
//         loop_end: &str,
//     ) -> Self {
//         Self {
//             rshift: rshift.to_string(),
//             lshift: lshift.to_string(),
//             inc: inc.to_string(),
//             dec: dec.to_string(),
//             write: write.to_string(),
//             read: read.to_string(),
//             loop_begin: loop_begin.to_string(),
//             loop_end: loop_end.to_string(),
//         }
//     }
// }
