use std::time::SystemTime;

use crate::drivers::Driver;

pub trait HasReceipt<D: Driver> {
    fn get_driver(&self) -> &D;
    fn get_reference_id(&self) -> &String;
    fn get_date(&self) -> &SystemTime;
}
