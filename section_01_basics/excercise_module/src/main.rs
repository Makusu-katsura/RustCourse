use excercise_module::config;
use excercise_module::models;
use excercise_module::services; 
fn main() {
  config::init();
    models::user::show_user();
    models::product::show_product();
    services::auth::login();
    services::payment::process_payment(100.0);
    services::auth::logout();
    println!("✅  Application finished successfully.");

}
