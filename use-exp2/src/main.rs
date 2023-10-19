use regex::Regex;

fn main() {
    let input = "opcode_time: Some({25: (2, 30), 153: (1, 15), 152: (1, 15), 90: (1, 15)}), cache_hits: (0, 0, 0, 0)";

    // let re = Regex::new(r"(\w+):\s*([^,]+(?:,\s*\w+:\s*[^,]+)*)").unwrap();
    // let re = Regex::new(r"(\w+):\s*((?:[^{}]+|(?R))+)(?:,|$)").unwrap();
    // let re = Regex::new(r"(\w+):\s*((?:\{(?:[^{}]|(?R))*\}|[^,])+)(?:,|$)").unwrap();
    // let re = Regex::new(r"(\w+):\s*((?:\{(?:[^{}]*|(?R))*\}|\([^()]*\)|[^,]+)+)(?:,|$)").unwrap();
    let re = Regex::new(r"(\w+):\s*((?:\{(?:[^{}]|(?R))*\}|(?:\([^()]*\))|[^,])+)(?:,|$)").unwrap();

    for capture in re.captures_iter(input) {
        let field_name = capture.get(1).unwrap().as_str();
        let field_value = capture.get(2).unwrap().as_str().trim();
        println!("Field Name: {}", field_name);
        println!("Field Value: {}", field_value);
    }
}