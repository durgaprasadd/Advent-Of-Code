fn checkRulesPart1(start: i32, end: i32) -> i32 {
    let mut number = start;
    let mut count = 0;
    while number < end {
        number += 1;
        if rule1(number) && rule2(number) {
            count += 1;
        }
    }
    return count;
}

fn checkRulesPart2(start: i32, end: i32) -> i32 {
    let mut number = start;
    let mut count = 0;
    while number < end {
        number += 1;
        if rule2(number) && rule3(number) {
            count += 1;
        }
    }
    return count;
}

//any two adjacent digits should be same
fn rule1(value: i32) -> bool {
    let mut i = value;
    let mut prevDigit = i % 10;
    let mut count = 1;
    while i > 1 {
        i /= 10;
        let currDigit = i % 10;
        if count == 2 {
            return true;
        }
        if currDigit == prevDigit {
            count += 1;
            continue;
        }
        count = 1;
        prevDigit = currDigit;
    }
    return false;
}

// only two adjacent digits should be same
fn rule3(value: i32) -> bool {
    let mut i = value;
    let mut prevDigit = i % 10;
    let mut count = 1;
    while i > 1 {
        i /= 10;
        let currDigit = i % 10;
        if currDigit == prevDigit {
            count += 1;
            continue;
        }
        if count == 2 {
            return true;
        }
        count = 1;
        prevDigit = currDigit;
    }
    return false;
}

// digits should be in increasing order
fn rule2(value: i32) -> bool {
    let mut i = value;
    let mut prevDigit = i % 10;
    while i > 10 {
        i /= 10;
        let currDigit = i % 10;
        if currDigit > prevDigit {
            return false;
        }
        prevDigit = currDigit;
    }
    return true;
}


fn main() {
    let start = 264360;
    let end = 746325;
    let count1 = checkRulesPart1(start, end);
    let count2 = checkRulesPart2(start, end);
    println!("count for part 1 is {}", count1);
    println!("count for part 2 is {}", count2);
}
