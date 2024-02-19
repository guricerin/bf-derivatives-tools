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

impl Grammar {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        rshift: &str,
        lshift: &str,
        inc: &str,
        dec: &str,
        write: &str,
        read: &str,
        loop_begin: &str,
        loop_end: &str,
    ) -> Self {
        Self {
            rshift: rshift.to_string(),
            lshift: lshift.to_string(),
            inc: inc.to_string(),
            dec: dec.to_string(),
            write: write.to_string(),
            read: read.to_string(),
            loop_begin: loop_begin.to_string(),
            loop_end: loop_end.to_string(),
        }
    }

    pub fn bf() -> Self {
        Self::new(">", "<", "+", "-", ".", ",", "[", "]")
    }
}
