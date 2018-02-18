extern crate eden;

fn main() {
    let client = eden::Eden::new();
    println!("{:?}", client.character().get_public_information(&client, 2112704025));
}