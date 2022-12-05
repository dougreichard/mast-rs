use super::utils::space;
use super::valid_id;
use super::MastCmd;
use super::json::value;
use pom::parser::{seq, sym};
use pom::parser::Parser;

pub(crate) fn match_command<'a>() -> Parser<'a, u8, MastCmd> {
    let condition  = valid_id() - sym(b':');
    seq(b"match") * space() * condition
        .map(|s| MastCmd::Match(s))
}


pub(crate) fn default_case_command<'a>() -> Parser<'a, u8, MastCmd> {
    (seq(b"case") * space() * sym(b'_') - sym(b':'))
        .map(|_|MastCmd::DefaultCase())
}


pub(crate) fn case_command<'a>() -> Parser<'a, u8, MastCmd> {
    let condition  = space().opt() *  value()  - sym(b':');
    seq(b"case") * space() * condition
        .map(|s| MastCmd::Case(s))
}

pub(crate) fn end_match_command<'a>() -> Parser<'a, u8, MastCmd> {
    seq(b"end_match").map(|s| MastCmd::EndMatch())
}


#[cfg(test)]
mod tests {
    #[test]
    fn match_test() {
        // One sapce
        let jump = super::match_command().parse(b"match x:");
        let _label: String = String::from("x");
        assert!(matches!(jump, Ok(super::MastCmd::Match(_label))));
       
    }
    #[test]
    fn case_test() {
        // One sapce
        let jump = super::case_command().parse(b"case 2:");
        let _label: String = String::from(" x>2");
        assert!(matches!(jump, Ok(super::MastCmd::Case(_label))));
        let jump = super::case_command().parse(b"case    2:");
        let _label: String = String::from("   x > 2 ");
        assert!(matches!(jump, Ok(super::MastCmd::Case(_label))));
    }
    #[test]
    fn default_test() {
        // One sapce
        let jump = super::default_case_command().parse(b"case _:");
        assert!(matches!(jump, Ok(super::MastCmd::DefaultCase())));
        
    }
    #[test]
    fn end_match_test() {
        // One sapce
        let jump = super::end_match_command().parse(b"end_match");
        assert!(matches!(jump, Ok(super::MastCmd::EndMatch())));
        
    }
}
