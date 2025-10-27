use jrust_std;

#[test]
fn test_print() {
    jrust_std::print("Hello from test");
}

#[test]
fn test_printf() {
    jrust_std::printf("Hello, {}!", &["World"]);
}
