pub fn reply(message: &str) -> &str {
    let is_empty = is_empty(message);
    
    if is_empty {
        return "Fine. Be that way!";
    }
    
    let is_question = is_question(message);
    let is_all_caps = is_all_caps(message);

    if is_question && is_all_caps {
        return "Calm down, I know what I'm doing!";
    } else if is_question {
        return "Sure.";
    } else {
        return "Whoa, chill out";
    }
}

fn is_question(message: &str) -> bool {
    let last_char = message.to_string()
        .trim()
        .chars()
        .nth(message.len() - 1);
    
    if let Some(last_char) = last_char {
        return last_char == '?';
    }
    return false;
}

fn is_all_caps(message: &str) -> bool {
    message.to_string()
           .chars()
           .all(|c| { 
               if c.is_alphabetic() { return c.is_uppercase(); }
               return true;
           })
}

fn is_empty(message: &str) -> bool {
    message.to_string().trim().is_empty()
}


