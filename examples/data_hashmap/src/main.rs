use std::collections::HashMap;

fn main() {
    let text = "hello world wonderfull world";
    let mut map = HashMap::new();
    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }
    println!("{:?}", map);

    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");
    //let field_name = "Favorite color";
    //let field_value = "Blue";
    let mut mapFields = HashMap::new();
    mapFields.insert(field_name, field_value);
    //println!("{} = {}", field_name, field_value);
}
