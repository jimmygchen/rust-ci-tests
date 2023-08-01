fn main() {}

#[test]
#[ignore]
fn test_boxed() {
    let _a = Box::new([-1; 3000000]);
}

#[test]
fn test_boxed_over_2mb() {
    // passes on linux and macos, fails on windows
    let _a = Box::new([0u8; 2 * 1024 * 1024 - 1024 * 9]);

    // passes on macos, fails on linux and windows
    // let _a = Box::new([0u8; 2 * 1024 * 1024 + 1024 * 8]);
}
