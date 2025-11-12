use std::collections::BTreeMap;
fn main() {
    let mut map = BTreeMap::new();
    map.insert(1, "One point zero");
    map.insert(2, "Two point zero");

    println!("{:?}", map);
}
