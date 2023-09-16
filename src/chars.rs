use lazy_regex::regex;

#[cfg(feature = "xid")]
pub fn is_ident_start(char: char) -> bool {
    unicode_ident::is_xid_start(char) || char == '_'
}

#[cfg(feature = "xid")]
pub fn is_ident_continue(char: char) -> bool {
    unicode_ident::is_xid_continue(char)
}

#[cfg(not(feature = "xid"))]
pub fn is_ident_start(char: char) -> bool {
    let regex = regex!("[a-zA-Z_]");
    regex.is_match(&char.to_string())
}

#[cfg(not(feature = "xid"))]
pub fn is_ident_continue(char: char) -> bool {
    let regex = regex!("[a-zA-Z0-9_]");
    regex.is_match(&char.to_string())
}

pub fn is_whitespace(char: char) -> bool {
    char.is_whitespace()
}

pub fn is_digit(char: char) -> bool {
    let regex = regex!("[0-9]");
    regex.is_match(&char.to_string())
}

pub fn is_bindigit(char: char) -> bool {
    let regex = regex!("[0-1]");
    regex.is_match(&char.to_string())
}

pub fn is_hexdigit(char: char) -> bool {
    let regex = regex!("[0-9a-fA-F]");
    regex.is_match(&char.to_string())
}

pub fn is_octdigit(char: char) -> bool {
    let regex = regex!("[0-7]");
    regex.is_match(&char.to_string())
}
