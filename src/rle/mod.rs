fn encode(input: &str) -> String {
    if input.is_empty() {
        return String::new();
    }
    let mut result = String::new();
    let mut chars = input.chars();
    let mut prev = chars.next().unwrap(); // get the first charater 
    let mut counter = 1;
    for c in chars {
        if c == prev { // see if the charaters are the same 
            counter += 1;
        } else {
            result.push(prev);
            if counter > 1 {
                result.push_str(&counter.to_string()); // we conclude the item before like 'a3'
            }
            prev = c;
            counter = 1;
        }
    } // for last number 
    result.push(prev);
    if counter > 1 {
        result.push_str(&counter.to_string());
    }
    result
}
// Now we will turn back the encoded back to the original from 
fn decode(input: &str) -> String {
    let mut ans = String::new();
    let mut chars = input.chars().peekable(); // Create a peekable iterator

    while let Some(c) = chars.next() {
        let mut Counting_String = String::new();
        while let Some(&next) = chars.peek() { // Peek ahead to see if there are digits
            if next.is_digit(10) {
                Counting_String.push(next); // if num. we push it in 
                chars.next();
            } else {
                break;
            }
        }
        let counter = if Counting_String.is_empty() { 1 } else { Counting_String.parse::<usize>().unwrap() };
        for _ in 0..counter {
            ans.push(c);
        }
    }
    ans
}
fn main() {
    let tests = vec![
        "aaabbbbbbbcdee", 
        "aaabcccdeee",    
        "",               
        "a",              
        "asdfghjkl",         
        "aaaaaaa",        
    ];
    for test in tests {
        let encoded = encode(test);
        let decoded = decode(&encoded);
        println!("Original: {}", test);
        println!("Encoded:  {}", encoded);
        println!("Decoded:  {}", decoded);
        assert_eq!(test, decoded);
        println!("Test passed!\n");
    }
}
