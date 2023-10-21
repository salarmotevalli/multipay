#[test]
fn test_unused() {
    let mut payment = super::payment::Payment::new();

    payment.amount(1000.0);
    payment.transaction_id("salar123456");
    let receipt = payment.verify(None);

    println!("{}", receipt.get_reference_id());

    assert_eq!("test_id", receipt.get_reference_id());
}
