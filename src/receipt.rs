use std::time::SystemTime;

pub struct Receipt<'a> {
    reference_id: &'a str,
    date: SystemTime,
}

impl<'a> Receipt<'a> {
    pub fn new(ref_id: &'a str) -> Self {
        Receipt {
            reference_id: ref_id,
            date: SystemTime::now(),
        }
    }

    pub fn get_date(&self) -> &SystemTime {
        &self.date
    }

    pub fn get_reference_id(&self) -> &'a str {
        self.reference_id
    }
}
