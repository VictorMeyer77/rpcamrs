use rpcamrs::rpcamrs::*;

fn main() {
    println!(
        "{:?}",
        single_capture("test.jpg", None, None, None).unwrap()
    );
}
