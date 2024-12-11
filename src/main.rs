use minet_ai::*;

fn main() {
    let minet = Minet::new(2, 4, 2);
    minet.display();
    minet.display_genome();
}