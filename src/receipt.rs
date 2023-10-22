use std::time::SystemTime;

pub struct Receipt {
    reference_id: &'static str,
    date: SystemTime,
}

impl Receipt {
    pub fn new(ref_id: &'static str) -> Self {
        Receipt {
            reference_id: ref_id,
            date: SystemTime::now(),
        }
    }

    pub fn get_date(&self) -> &SystemTime {
        &self.date
    }

    pub fn get_reference_id(&self) -> &'static str {
        self.reference_id
    }
}
