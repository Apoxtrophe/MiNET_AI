use minet_ai::*;
use rand::Rng;

fn main() {
    let mut minet = Minet::new(2, 2, 1);
    
    minet.display();
    
    minet.dot_to_file("minet_test.dot");
  
}