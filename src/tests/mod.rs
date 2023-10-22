use std::collections::HashMap;

#[test]
fn test_it_has_default_driver() {
    // let mut payment = super::payment::Payment::new();

    // payment.amount(1000.0);
    // payment.transaction_id("salar123456");
    // let receipt = payment.verify(None);

    // println!("{}", receipt.get_reference_id());

    // assert_eq!("test_id", receipt.get_reference_id());
}

#[test]
fn test_it_wont_accept_invalid_driver() {
    #[derive(Eq, Hash, PartialEq)]
    enum ttt {
        A,
        B,
        C,
    }
    let mut data = HashMap::<String, String>::new();
    data.insert("s".into(), "saa".to_string());
    data.insert("s".into(), "sala".to_string());

    println!("{data:?}");
}

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
