pub fn raindrops(n: u32) -> String {
    let mut result: String = String::from("");

    let (add_pling, add_plang, add_plong) = (n % 3 == 0, n % 5 == 0, n % 7 == 0);

    if add_pling {
        result.push_str("Pling");
    }
    if add_plang {
        result.push_str("Plang");
    }
    if add_plong {
        result.push_str("Plong");
    }

    if result == "" {
        return n.to_string();
    }
    return result;
}

pub fn raindrops2(x: u32) -> String {
    let is_factor = |factor| x % factor == 0;
    let mut rez = String::new();

    if is_factor(3) { rez.push_str("Pling"); }
    if is_factor(5) { rez.push_str("Plang"); }
    if is_factor(7) { rez.push_str("Plong"); }

    if rez.is_empty() { rez = x.to_string(); }

    rez
}