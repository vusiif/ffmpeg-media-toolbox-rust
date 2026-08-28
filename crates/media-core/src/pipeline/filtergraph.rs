#[derive(Debug, Clone, Default)]
pub struct FilterGraph {
    filters: Vec<String>,
}

impl std::fmt::Display for FilterGraph {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.filters.join(","))
    }
}

impl FilterGraph {
    pub fn new() -> Self {
        Self {
            filters: Vec::new(),
        }
    }

    pub fn push(&mut self, filter: String) {
        self.filters.push(filter);
    }

    pub fn is_empty(&self) -> bool {
        self.filters.is_empty()
    }

    pub fn to_filter_complex(&self) -> Vec<String> {
        if self.filters.is_empty() {
            return vec![];
        }
        vec!["-vf".to_string(), self.to_string()]
    }
}
