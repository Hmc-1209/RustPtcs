fn main() {
  let mut account: BankAccount = BankAccount{
    owner: "Danny".to_string(),
    balance: 100.0,
  };

  account.check_balance();
  account.withdraw(45.3);
  account.check_balance();
}

struct BankAccount {
  owner: String,
  balance: f64,
}
impl BankAccount {
  fn withdraw(&mut self, amount: f64) {
    println!("Withdrawing from {} with balance {}.", self.owner, amount);
    self.balance -= amount;
  }

  fn check_balance(&self) {
    println!("Account {} has a balance of {}", self.owner, self.balance);
  }
}