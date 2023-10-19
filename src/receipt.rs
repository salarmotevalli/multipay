use std::time::SystemTime;

use crate::{drivers::Driver, traits::has_receipt::HasReceipt};

pub struct Receipt<D: Driver> {
    reference_id: String,
    driver: D,
    date: SystemTime,
}

impl<D: Driver> HasReceipt<D> for Receipt<D>{
    fn get_date(&self) -> &SystemTime {
        &self.date
    }

    fn get_driver(&self) -> &D  {
        &self.driver
    }

    fn get_reference_id(&self) -> &String {
        &self.reference_id
    }
}

impl<D: Driver> Receipt<D> {
    pub fn new(ref_id: String, driver: D) -> Self 
        where D: Driver,
    {
        Receipt {
            reference_id: ref_id,
            driver,
            date: SystemTime::now(),
        }
    }
}


