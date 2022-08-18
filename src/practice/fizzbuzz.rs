pub fn fizzbuzz(max: i32) -> Vec<String> {
    let mut out = Vec::new();

    for i in 1..=max {
        if i % 3 != 0 && i % 5 != 0 {
            out.push(i.to_string());
            continue;
        }
        
        let mut s = String::new();
        if i % 3 == 0 {
            s += "Fizz";
        }

        if i % 5 == 0 {
            s += "Buzz";
        }

        out.push(s);
    }

    out
}