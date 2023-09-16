pub mod chacacter_buffer;
pub mod chars;
pub use chacacter_buffer::{CharacterBuffer, Cursor};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_character_buffer() {
        let data = "Awesome\ndemo";
        let mut source = CharacterBuffer::new(std::io::Cursor::new(data));
        assert_eq!(source.peek_char(), Some('A'));
        assert_eq!(source.next_char(), Some('A'));
        assert_eq!(source.next_char_if(|char| char == 'e'), None);
        source.next_char();
        assert_eq!(source.next_char_if(|char| char == 'e'), Some('e'));
        assert_eq!(source.cursor().line, 1);
        assert_eq!(source.cursor().column, 3);
        for _ in 0..5 {
            source.next_char();
        }
        assert_eq!(source.cursor().line, 2);
        assert_eq!(source.cursor().column, 1);
        assert_eq!(source.next_token(char::is_alphabetic, '1'), "1demo");
        assert_eq!(source.peek_char(), None);
        assert_eq!(source.next_char(), None);
    }

    #[test]
    fn test_chars() {
        assert!(chars::is_whitespace(' '));
        assert!(chars::is_whitespace('\n'));
        assert!(chars::is_whitespace('\t'));
        assert!(chars::is_whitespace('\r'));

        assert!(!chars::is_whitespace('a'));
        assert!(!chars::is_whitespace('_'));

        assert!(chars::is_ident_start('D'));
        assert!(chars::is_ident_start('f'));
        assert!(chars::is_ident_start('_'));

        assert!(!chars::is_ident_start('!'));
        assert!(!chars::is_ident_start(' '));
        assert!(!chars::is_ident_start('4'));

        assert!(chars::is_ident_continue('D'));
        assert!(chars::is_ident_continue('f'));
        assert!(chars::is_ident_continue('_'));
        assert!(chars::is_ident_continue('4'));

        assert!(!chars::is_ident_continue('!'));
        assert!(!chars::is_ident_continue(' '));

        #[cfg(feature = "xid")]
        {
            assert!(chars::is_ident_start('Ф'));
            assert!(chars::is_ident_start('ф'));
            assert!(chars::is_ident_start('\u{1000d}'));
            assert!(chars::is_ident_start('\u{10026}'));

            assert!(!chars::is_ident_start('\u{02c2}'));
            assert!(!chars::is_ident_start('\u{ffff}'));

            assert!(chars::is_ident_continue('\u{1000d}'));
            assert!(chars::is_ident_continue('\u{10026}'));

            assert!(!chars::is_ident_continue('\u{02c2}'));
            assert!(!chars::is_ident_continue('\u{ffff}'));
        }

        assert!(chars::is_digit('0'));
        assert!(chars::is_digit('4'));
        assert!(chars::is_digit('8'));
        assert!(!chars::is_digit('s'));
        assert!(!chars::is_digit(' '));
        assert!(!chars::is_digit('/'));

        assert!(chars::is_bindigit('0'));
        assert!(chars::is_bindigit('1'));
        assert!(!chars::is_bindigit('2'));
        assert!(!chars::is_bindigit(' '));
        assert!(!chars::is_bindigit('/'));

        assert!(chars::is_hexdigit('3'));
        assert!(chars::is_hexdigit('f'));
        assert!(chars::is_hexdigit('F'));
        assert!(!chars::is_hexdigit('g'));
        assert!(!chars::is_hexdigit(' '));
        assert!(!chars::is_hexdigit('/'));

        assert!(chars::is_octdigit('0'));
        assert!(chars::is_octdigit('4'));
        assert!(chars::is_octdigit('7'));
        assert!(!chars::is_octdigit('8'));
        assert!(!chars::is_octdigit('f'));
        assert!(!chars::is_octdigit('/'));
    }
}
