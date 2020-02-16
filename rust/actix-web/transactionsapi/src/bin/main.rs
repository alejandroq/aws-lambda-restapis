fn main() {
    actix_lambda::run(transactionsapi::config);
}

#[test]
fn lambda_test() {
    actix_lambda::test::lambda_test(main);
}