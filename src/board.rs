use field::Field;

pub const LENGTH: usize = 64;

pub enum Player {
    None,
    CPU,
    Human,
}

pub struct Board {
    fields: Vec<Field>
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