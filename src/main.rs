use std::collections::HashMap;

fn two_sum(vec: Vec<i32>, target: i32) -> Vec<i32> {
    for i in 0..vec.len() {
        for j in i + 1..vec.len() {
            if vec[i] + vec[j] == target {
                return vec![i as i32, j as i32];
            }
        }
    }
    vec![]
}

fn is_palindrome(x: i32) -> bool {
    let mut reversed_string = String::new();
    let num_as_string = x.to_string();
    let mut string_len = num_as_string.len();

    while string_len > 0 {
        reversed_string.push(num_as_string.chars().nth(string_len - 1).unwrap());
        string_len -= 1;
    }

    if num_as_string == reversed_string {
        return true;
    }

    return false;
}

fn roman_to_int(s: String) -> i32 {
    // I can be placed before V (5) and X (10) to make 4 and 9.
    // X can be placed before L (50) and C (100) to make 40 and 90.
    // C can be placed before D (500) and M (1000) to make 400 and 900.
    let mut roman_numerals = HashMap::new();
    roman_numerals.insert('I', 1);
    roman_numerals.insert('V', 5);
    roman_numerals.insert('X', 10);
    roman_numerals.insert('L', 50);
    roman_numerals.insert('C', 100);
    roman_numerals.insert('D', 500);
    roman_numerals.insert('M', 1000);

    let mut result = 0;
    let mut i = 0;

    while i < s.len() {
        let current_char = s.chars().nth(i).unwrap();
        let mut next_char: char = 'C';

        if (i + 1) < s.len() {
            next_char = s.chars().nth(i + 1).unwrap();
        }

        let current_value = roman_numerals.get(&current_char).unwrap();
        let next_value = roman_numerals.get(&next_char).unwrap();

        if current_value < next_value {
            result += next_value - current_value;
            i += 1;
        } else {
            result += current_value;
        }

        i += 1;
    }

    return result;
}

fn longest_common_prefix(strs: Vec<String>) -> String {
    let mut prefixes_map: HashMap<String, i32> = HashMap::new();
    let mut longest_prefix = String::new();

    for i in 0..strs.len() {
        let mut prefix = String::new();
        let current_string = &strs[i];

        for current_char in current_string.chars() {
            prefix.push(current_char);

            if !prefixes_map.contains_key(&prefix) {
                prefixes_map.insert(prefix.clone(), 1);
            } else {
                let count = prefixes_map.get(&prefix).unwrap();
                prefixes_map.insert(prefix.clone(), count + 1);
            }
        }
    }

    for (prefix, count) in prefixes_map.iter() {
        if *count == strs.len() as i32 && prefix.len() > longest_prefix.len() {
            longest_prefix = prefix.clone();
        }
    }

    return longest_prefix;
}

fn validate_parantheses(s: String) -> bool {
    // (){}}{ -> false
    // (){}[] -> true
    // (] -> false
    // [ -> false
    // () -> true
    // (( -> false
    // {[]} -> true
    // ([]){ -> false

    if s.len() == 0 || s.len() == 1 {
        return false;
    }

    let mut stack: Vec<char> = Vec::new();
    let s_chars = s.chars();

    for current_char in s_chars {
        let last_char = stack.last();

        if current_char == '{' || current_char == '(' || current_char == '[' {
            // if last char is opening bracket and current char is also opening bracket and is the last char in s return false
            if 
                last_char.is_some() && 
                (last_char == Some(&'{') || last_char == Some(&'(') || last_char == Some(&'[')) &&
                s.chars().last() == Some(current_char) 
            {
                return false;
            }
            stack.push(current_char);
        } else if current_char == '}' || current_char == ')' || current_char == ']' {
            if last_char.is_none() {
                return false;
            }

            if (current_char == '}' && last_char == Some(&'{'))
                || (current_char == ')' && last_char == Some(&'('))
                || (current_char == ']' && last_char == Some(&'['))
            {
                stack.pop();
            } else {
                return false;
            }

            if stack.len() == 0 {
                continue;
            }
        } else {
            return false
        }
    }
    return true;
}

fn main() {
    println!("Two Sum Start ======================");
    let input_vec = vec![2, 7, 11, 15];
    let target = 9;
    let result = two_sum(input_vec, target);
    assert_eq!(result, vec![0, 1]);
    println!("Two Sum result -> {:?}", result);
    println!("Two Sum End ======================");
    println!();

    println!("Is Palindrome Start ======================");
    let input_num = 121;
    let result = is_palindrome(input_num);
    assert_eq!(result, true);
    println!("Is Palindrome result -> {:?}", result);
    println!("Is Palindrome End ======================");
    println!();

    println!("Roman to Integer Start ======================");
    let input_string = String::from("MCMXCIV");
    let result = roman_to_int(input_string);
    assert_eq!(result, 1994);
    println!("Roman integer conversion result -> {:?}", result);
    println!("Roman to Integer End ======================");
    println!();

    println!("Longest Common Prefix Start ======================");
    let input_vec = vec![
        String::from("flower"),
        String::from("flow"),
        String::from("flight"),
    ];
    let result = longest_common_prefix(input_vec);
    assert_eq!(result, String::from("fl"));
    println!("Largest common prefix -> {:?}", result);
    println!("Longest Common Prefix End ======================");
    println!();


    println!("Validate Parantheses Start ======================");
    let input_string = String::from("(){}}{");
    let result = validate_parantheses(input_string);
    assert_eq!(result, false, "we are testing with {} should return {}", result, false);
    println!("Is valid parantheses () {{}} }}{{ -> {:?}", result);
    let input_string = String::from("(){}[]");
    let result = validate_parantheses(input_string);
    assert_eq!(result, true, "we are testing with {} should return {}", result, true);
    println!("Is valid parantheses () {{}} [] -> {:?}", result);
    let input_string = String::from("(]");
    let result = validate_parantheses(input_string);
    assert_eq!(result, false, "we are testing with {} should return {}", result, false);
    println!("Is valid parantheses (] -> {:?}", result);
    let input_string = String::from("[");
    let result = validate_parantheses(input_string);
    assert_eq!(result, false, "we are testing with {} should return {}", result, false);
    println!("Is valid parantheses [ -> {:?}", result);
    let input_string = String::from("()");
    let result = validate_parantheses(input_string);
    assert_eq!(result, true, "we are testing with {} should return {}", result, true);
    println!("Is valid parantheses () -> {:?}", result);
    let input_string = String::from("((");
    let result = validate_parantheses(input_string);
    assert_eq!(result, false, "we are testing with {} should return {}", result, false);
    println!("Is valid parantheses (( -> {:?}", result);
    let input_string = String::from("{[]}");
    let result = validate_parantheses(input_string);
    assert_eq!(result, true, "we are testing with {} should return {}", result, true);
    println!("Is valid parantheses {{[]}} -> {:?}", result);
    let input_string = String::from("([]){");
    let result = validate_parantheses(input_string);
    assert_eq!(result, false, "we are testing with `([]){{` {} should return {}", result, false);
    println!("Is valid parantheses ([]){{ -> {:?}", result);
    println!("Validate Parantheses End ======================");
    println!();
}
