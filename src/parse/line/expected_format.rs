pub struct ExpectedFormat {
    pub format: String, // format without keyword
    pub size: usize,    // number of tokens in format	(without keyword)
}

impl ExpectedFormat {
    pub fn new(keyword: &'static str, expected_format: Option<impl Into<String>>) -> Self {
        let Some(expected_format) = expected_format else {
			return Self {
				format: String::new(),
				size: 0,
			};
		};

        let mut format: String = expected_format.into();
        if format.starts_with(keyword) {
            let format = format.replacen(keyword, "", 1).trim().to_string();
        }

        let size = format
            .split_whitespace()
            .into_iter()
            .collect::<Vec<&str>>()
            .len();

        Self { format, size }
    }
}
