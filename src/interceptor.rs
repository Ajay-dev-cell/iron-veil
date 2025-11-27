use crate::protocol::postgres::{DataRow, RowDescription};
use anyhow::Result;
use fake::faker::internet::en::SafeEmail;
use fake::Fake;

pub trait PacketInterceptor {
    fn on_row_description(&mut self, msg: &RowDescription);
    fn on_data_row(&mut self, msg: DataRow) -> Result<DataRow>;
}

pub struct Anonymizer {
    // Map of column index to masking strategy
    // For now, we'll just hardcode it.
    target_col_indices: Vec<usize>,
}

impl Anonymizer {
    pub fn new() -> Self {
        Self {
            target_col_indices: Vec::new(),
        }
    }
}

impl PacketInterceptor for Anonymizer {
    fn on_row_description(&mut self, msg: &RowDescription) {
        self.target_col_indices.clear();
        for (i, field) in msg.fields.iter().enumerate() {
            // Hardcoded rule: mask "email" column
            if field.name == "email" {
                self.target_col_indices.push(i);
            }
        }
    }

    fn on_data_row(&mut self, mut msg: DataRow) -> Result<DataRow> {
        if self.target_col_indices.is_empty() {
            return Ok(msg);
        }

        for &idx in &self.target_col_indices {
            if idx < msg.values.len() {
                if let Some(val) = &mut msg.values[idx] {
                    // Replace with fake email
                    let fake_email: String = SafeEmail().fake();
                    val.clear();
                    val.extend_from_slice(fake_email.as_bytes());
                }
            }
        }
        Ok(msg)
    }
}
