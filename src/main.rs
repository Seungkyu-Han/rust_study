trait Taxable {
    const TAX_RATE: f64 = 0.25;

    fn amount(&self) -> f64;

    fn tax_bill(&self) -> f64 {
        self.amount() * Self::TAX_RATE
    }
}

#[derive(Debug)]
struct Income {
    amount: f64,
}

impl Taxable for Income {
    fn amount(&self) -> f64 {
        self.amount
    }
}

#[derive(Debug)]
struct Bonus {
    amount: f64,
}

impl Taxable for Bonus {
    const TAX_RATE: f64 = 0.5;

    fn amount(&self) -> f64 {
        self.amount
    }
}

fn main() {
    let income = Income { amount: 500.00 };

    println!("Total tax owed: ${:.2}", income.tax_bill());

    let bonus = Bonus { amount: 100.0 };
    println!("Bonus tax owed: ${:.2}", bonus.tax_bill());
}
