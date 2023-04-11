pub struct Keyword {
    pub keyword: &'static str,
    pub desc: &'static str,
}

impl Keyword {
    pub fn new(keyword: &'static str, desc: &'static str) -> Self {
        Self { keyword, desc }
    }
}
