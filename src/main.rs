use minet_ai::*;
use rand::Rng;

fn main() {
    let mut minet = Minet::new(2, 4, 2);
    minet.display();
    minet.dot_to_file("minet_test.dot");

}