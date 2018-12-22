#[macro_use]
extern crate serde_json;

fn main() {
    // The type of `john` is `serde_json::Value`
    let john = json!({
      "name": "John Doe",
      "age": 43,
      "phones": [
        "+44 1234567",
        "+44 2345678"
      ]
    });
  
    println!("first phone number: {}", john["phones"][0]);

    // Convert to a string of JSON and print it out
    println!("json data {}", john.to_string());
}