fn encode(input: &str) -> String {
    if input.is_empty() {
        return String::new();
    }
    let mut result = String::new();
    let mut chars = input.chars();
    let mut prev = chars.next().unwrap(); // get the first charater 
    result.push(prev); 
    result
}

fn main() {
    println!("Test TEST");
}
