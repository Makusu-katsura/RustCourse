mod calculator;
fn main() {
  let apple_price = 15.5;
  let apple_quantity = 5;
  let banana_price = 10.0;
  let banana_quantity = 3;

  let apple_total = calculator::calculate_item_total(apple_price, apple_quantity);
  let banana_total = calculator::calculate_item_total(banana_price, banana_quantity);

  println!("Total price for apples: {}", apple_total);
  println!("Total price for bananas: {}", banana_total);

  let grand_total = apple_total + banana_total;
  println!("Grand total: {}", grand_total); 
}
