use rocket_lamb::RocketExt;

fn main() {
    transactionsapi::rocket().lambda().launch();
}
