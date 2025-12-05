use regex::Regex;

#[derive(Debug, Clone, PartialEq)]
pub enum PiiType {
    Email,
    CreditCard,
}

pub struct PiiScanner {
    email_regex: Regex,
    cc_regex: Regex,
}

impl Default for PiiScanner {
    fn default() -> Self {
        Self::new()
    }
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_email_detection() {
        let scanner = PiiScanner::new();

        // Valid emails
        assert_eq!(scanner.scan("test@example.com"), Some(PiiType::Email));
        assert_eq!(scanner.scan("john.doe@company.org"), Some(PiiType::Email));
        assert_eq!(scanner.scan("user+tag@domain.co.uk"), Some(PiiType::Email));
        assert_eq!(scanner.scan("USER@EXAMPLE.COM"), Some(PiiType::Email));

        // Invalid emails
        assert_eq!(scanner.scan("not-an-email"), None);
        assert_eq!(scanner.scan("missing@domain"), None);
        assert_eq!(scanner.scan("@nodomain.com"), None);
        assert_eq!(scanner.scan("spaces in@email.com"), None);
    }

    #[test]
    fn test_credit_card_detection() {
        let scanner = PiiScanner::new();

        // Valid credit cards
        assert_eq!(
            scanner.scan("1234-5678-9012-3456"),
            Some(PiiType::CreditCard)
        );
        assert_eq!(
            scanner.scan("1234 5678 9012 3456"),
            Some(PiiType::CreditCard)
        );
        assert_eq!(scanner.scan("1234567890123456"), Some(PiiType::CreditCard));

        // Invalid credit cards
        assert_eq!(scanner.scan("1234-5678-9012"), None);
        assert_eq!(scanner.scan("not a credit card"), None);
        assert_eq!(scanner.scan("12345678901234567890"), None); // Too long
    }

    #[test]
    fn test_non_pii_data() {
        let scanner = PiiScanner::new();

        assert_eq!(scanner.scan("John Doe"), None);
        assert_eq!(scanner.scan("123 Main Street"), None);
        assert_eq!(scanner.scan("Hello, World!"), None);
        assert_eq!(scanner.scan(""), None);
        assert_eq!(scanner.scan("12345"), None);
    }

    #[test]
    fn test_default_trait() {
        let scanner = PiiScanner::default();
        assert_eq!(scanner.scan("test@example.com"), Some(PiiType::Email));
    }
}
