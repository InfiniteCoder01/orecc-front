use lazy_regex::regex;

/// Checks if the given character can be the first character of identifier
#[cfg(feature = "xid")]
pub fn is_ident_start(char: char) -> bool {
    unicode_ident::is_xid_start(char) || char == '_'
}

/// Checks if the given character can be the character of identifier (not the first character)
#[cfg(feature = "xid")]
pub fn is_ident_continue(char: char) -> bool {
    unicode_ident::is_xid_continue(char)
}

/// Checks if the given character can be the first character of identifier
#[cfg(not(feature = "xid"))]
pub fn is_ident_start(char: char) -> bool {
    let regex = regex!("[a-zA-Z_]");
    regex.is_match(&char.to_string())
}

/// Checks if the given character can be the character of identifier (not the first character)
#[cfg(not(feature = "xid"))]
pub fn is_ident_continue(char: char) -> bool {
    let regex = regex!("[a-zA-Z0-9_]");
    regex.is_match(&char.to_string())
}

/// Checks if the given character is a whitespace, just calls [`char::is_whitespace()`]
pub fn is_whitespace(char: char) -> bool {
    char.is_whitespace()
}

/// Checks if the character is a digit (0 through 9)
pub fn is_digit(char: char) -> bool {
    let regex = regex!("[0-9]");
    regex.is_match(&char.to_string())
}

/// Checks if the character is a binary digit (0 or 1)
pub fn is_bindigit(char: char) -> bool {
    let regex = regex!("[0-1]");
    regex.is_match(&char.to_string())
}

/// Checks if the character is a hexadecimal digit (0 through 9, a through f and A through F)
pub fn is_hexdigit(char: char) -> bool {
    let regex = regex!("[0-9a-fA-F]");
    regex.is_match(&char.to_string())
}

/// Checks if the character is an octal digit (0 through 7)
pub fn is_octdigit(char: char) -> bool {
    let regex = regex!("[0-7]");
    regex.is_match(&char.to_string())
}
