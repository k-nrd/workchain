pub fn hex_to_binary(hex: &str) -> String {
    hex.chars()
        .map(|x| to_binary(x).expect("Error converting hex to binary"))
        .collect()
}

fn to_binary(c: char) -> Option<&'static str> {
    match c {
        '0' => Some("0000"),
        '1' => Some("0001"),
        '2' => Some("0010"),
        '3' => Some("0011"),
        '4' => Some("0100"),
        '5' => Some("0101"),
        '6' => Some("0110"),
        '7' => Some("0111"),
        '8' => Some("1000"),
        '9' => Some("1001"),
        'A' | 'a' => Some("1010"),
        'B' | 'b' => Some("1011"),
        'C' | 'c' => Some("1100"),
        'D' | 'd' => Some("1101"),
        'E' | 'e' => Some("1110"),
        'F' | 'f' => Some("1111"),
        _ => None,
    }
}
