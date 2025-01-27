use minet_ai::*;
use rand::Rng;

fn main() {
    let mut minet = Minet::new(1, 4, 1);
    
    minet.display();
    minet.dot_to_file("minet_test.dot");

}