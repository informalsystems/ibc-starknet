pub fn digit_to_u4(value: u8) -> u8 {
    if '0' <= value && value <= '9' {
        value - '0'
    } else if 'A' <= value && value <= 'F' {
        value - 'A' + 10
    } else if 'a' <= value && value <= 'f' {
        value - 'a' + 10
    } else {
        panic!("Invalid u4 hex digit: {}", value)
    }
}

pub fn u4_to_upper_digit(value: u8) -> u8 {
    if value < 10 {
        '0' + value
    } else if value < 16 {
        // always uppercase
        'A' + value - 10
    } else {
        panic!("Invalid u4 hex value: {}", value)
    }
}

pub fn u4_to_lower_digit(value: u8) -> u8 {
    if value < 10 {
        '0' + value
    } else if value < 16 {
        // always lowercase
        'a' + value - 10
    } else {
        panic!("Invalid u4 hex value: {}", value)
    }
}

