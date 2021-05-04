pub fn is_leap_year(year: u64) -> bool {
    let divisible_by_4 = year % 4 == 0;
    let divisible_by_100 = year % 100 == 0;
    let divisible_by_400 = year % 400 == 0;

    if !divisible_by_4 {
        return false;
    }
    if divisible_by_100 {
        return divisible_by_400;
    }
    return true;
}

pub fn is_leap_year_2(year: i64) -> bool {
    match (year % 4, year % 100, year % 400) {
        (0, 0, 0) => true,
        (0, 0, _) => false,
        (0, _, _) => true,
        (_, _, _) => false,
    }
}