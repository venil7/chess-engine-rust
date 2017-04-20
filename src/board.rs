use std::fmt;
use field::Field;

pub const LENGTH: usize = 64;

pub enum Player {
    None,
    CPU,
    Human,
}

pub struct Board {
    fields: Vec<Field>,
}

impl Board {
    pub fn new() -> Board {
        let fields = (0..LENGTH).map(|_| Field::empty()).collect();
        Board { fields: fields }
    }

    pub fn fields(&self) -> &Vec<Field> {
        &self.fields
    }
}

impl fmt::Display for Board {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        let str = self.fields()
            .iter()
            .enumerate()
            .fold("".to_string(), |acc, (idx, field)| {
                let mut result = acc + &field.to_string();
                if (idx + 1) % 8 == 0 {
                    result = result + &"\n"
                }
                result
            });
        write!(formatter, "{}", str)
    }
}