const OPEN_BRACKETS: [char; 3] = ['(', '[', '{'];
const CLOSED_BRACKETS: [char; 3] = [')', ']', '}'];

pub fn brackets_are_balanced(string: &str) -> bool {
    // place to store open brackets
    let mut stack: Vec<char> = Vec::new();
    // iterate over characters in input string
    for chr in string.chars() {
        // if character is open bracket store it into stack
        if OPEN_BRACKETS.contains(&chr) {
            stack.push(chr);
        }
        // if character is closed bracket
        if let Some(idx) = CLOSED_BRACKETS.iter().position(|&x| x == chr) {
            // find corresponding open bracket
            let open_chr = OPEN_BRACKETS[idx];
            // get last added open bracket from stack
            if let Some(stack_chr) = stack.pop() {
                // fail if it is not equal to expected open bracket
                if stack_chr != open_chr {
                    return false;
                }
            // or fail
            } else {
                return false;
            }
        }
    }
    // fail if stack is not empty
    return stack.len() == 0;
}
