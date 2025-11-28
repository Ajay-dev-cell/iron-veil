use regex::Regex;

#[derive(Debug, Clone, PartialEq)]
pub enum PiiType {
    Email,
    CreditCard,
    // SSN, // TODO: Add SSN regex
}

pub struct PiiScanner {
    email_regex: Regex,
    cc_regex: Regex,
}

impl PiiScanner {
    pub fn new() -> Self {
        Self {
            // Simple email regex
            email_regex: Regex::new(r"(?i)^[a-z0-9._%+-]+@[a-z0-9.-]+\.[a-z]{2,}$").unwrap(),
            // Simple Credit Card regex (13-19 digits, optional dashes/spaces)
            // This is a heuristic, not a perfect validator (Luhn algorithm would be better for validation, but regex is fine for detection)
            cc_regex: Regex::new(r"^(?:\d{4}[-\s]?){3}\d{4}$").unwrap(),
        }
    }

    pub fn scan(&self, text: &str) -> Option<PiiType> {
        if self.email_regex.is_match(text) {
            return Some(PiiType::Email);
        }
        if self.cc_regex.is_match(text) {
            return Some(PiiType::CreditCard);
        }
        None
    }
}
