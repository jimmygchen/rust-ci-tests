use tokio::runtime::Builder;

fn main() {}

#[test]
fn test_boxed() {
    let a = Box::new([-1; 3000000]);
}
