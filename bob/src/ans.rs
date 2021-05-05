pub fn reply(statement : &str) -> String {

    let is_question = |words: &str| words.ends_with("?");
    let is_saying_nothing = |words: &str| words.is_empty();
    let is_yelling = |words: &str| words == words.to_uppercase();
    
    if is_question(statement) {
        "Sure.".to_string()
    } else if is_saying_nothing(statement) {
        "Fine. Be that way!".to_string()
    } else if is_yelling(statement) {
        "Whoa, chill out!".to_string()
    } else {
        "Whatever.".to_string()
    }
}