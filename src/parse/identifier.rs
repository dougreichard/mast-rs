use pom::char_class::{alpha, alphanum};
use pom::parser::{is_a, Parser};

#[inline]
pub fn var_start(term: u8) -> bool {
	alpha(term) || term == b'_'
}
#[inline]
pub fn var_rest(term: u8) -> bool {
	alphanum(term) || term == b'_'
}
pub(crate) fn valid_id<'a>() -> Parser<'a, u8, String> {
    (is_a(var_start) + is_a(var_rest).repeat(0..))
        .map(|(first, rest)| format!("{}{}", first as char, String::from_utf8(rest).unwrap()))
}



#[cfg(test)]
mod tests {
    #[test]
    fn identifier_test() {
        let jump = super::valid_id().parse(b"MyLabel");
        let _label: String = String::from("MyLabel");
        assert!(matches!(jump, Ok(_label)));

        let _label: String = String::from("_MyLabel");
        let jump = super::valid_id().parse(_label.as_bytes());
        assert!(matches!(jump, Ok(_label)));

        let _label: String = String::from("my_label_99");
        let jump = super::valid_id().parse(_label.as_bytes());
        assert!(matches!(jump, Ok(_label)));
    }
}
