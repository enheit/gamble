#[derive(Debug)]
enum Status {
    Declined,
    Cancelled
}

fn main () {
    println!("{:?}", Status::Declined);
}
