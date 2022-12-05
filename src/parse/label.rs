use pom::parser::sym;
use super::MastCmd;
use super::utils::space;
use super::identifier::valid_id;
use pom::parser::Parser;

pub(crate) fn label<'a>() -> Parser<'a, u8, String> {
    let valid_id = valid_id();
    sym(b'=').repeat(2..) * space() * valid_id.map(|s| s)
        - space()
        - sym(b'=').repeat(2..)
}



#[cfg(test)]
mod tests {
    #[test]
    fn label_test() {
        let jump = super::label().parse(b"==MyLabel==");
        let _label: String = String::from("MyLabel");
        assert!(matches!(jump, Ok(_label)));

        let jump = super::label().parse(b"== MyLabel==");
        let _label: String = String::from("MyLabel");
        assert!(matches!(jump, Ok(_label)));
        let jump = super::label().parse(b"== MyLabel ==");
        let _label: String = String::from("MyLabel");
        assert!(matches!(jump, Ok(_label)));
        let jump = super::label().parse(b"====== MyLabel ==");
        let _label: String = String::from("MyLabel");
        assert!(matches!(jump, Ok(_label)));
        let jump = super::label().parse(b"== MyLabel =======");
        let _label: String = String::from("MyLabel");
        assert!(matches!(jump, Ok(_label)));


    }
}
