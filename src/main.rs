#[derive(Debug)]
enum PaymentMethodType {
    CreditCard(String),
    DebitCard(String),
    PayPal(String),
}

fn main() {
    let visa = PaymentMethodType::CreditCard(String::from("dddd"));
    println!("{:?}", visa);
}
