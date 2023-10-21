#[test]
fn test_unused() {

    let mut payment = super::payment::Payment::new();

    payment.amount(1000.0);
    payment.transaction_id("salar123456");
}
