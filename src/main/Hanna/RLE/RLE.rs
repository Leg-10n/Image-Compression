fn encode(input: &str) -> String {
    if input.is_empty() {
        return String::new();
    }
    let mut result = String::new();
    let mut chars = input.chars();
    let mut prev = chars.next().unwrap(); // get the first charater 
    let mut counter = 1;
    for c in chars {
        if c == prev {
            counter += 1;
        } else {
            result.push(prev);
            if counter > 1 {
                result.push_str(&counter.to_string());
            }
            prev = c;
            counter = 1;
        }
    }
    result.push(prev);
    if count > 1 {
        result.push_str(&count.to_string());
    }
    result
}
fn decode(input: &str) ->String {
    ???
}
fn main() {
    println!("Test TEST");
}
