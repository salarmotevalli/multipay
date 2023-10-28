use crate::{drivers::zarinpal::ZarinPal, invoice::Invoice, payment::Payment};

#[test]
fn test_init_payment_with_driver() {
    let mut invoice: Invoice = Invoice::new(1000.0);
    invoice.detail("name1", "salar");
    invoice.detail("name2", "motevalli");
    let mut payment: Payment<ZarinPal> = Payment::new(invoice);

    payment.purchase(Some(|_drv, _trs| {}));
    payment.pay(None);
}

#[test]
fn test_it_wont_accept_invalid_driver() {}

#[test]
fn config_can_be_modified() {}

#[test]
fn callback_url_can_be_modified() {}

#[test]
fn amount_can_be_setted() {}

#[test]
fn driver_can_be_changed() {}

#[test]
fn test_purchase() {}

#[test]
fn test_custom_invoice_can_be_used_in_purchase() {}

#[test]
fn test_pay() {}
