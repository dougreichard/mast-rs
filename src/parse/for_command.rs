use super::utils::space;
use super::valid_id;
use super::MastCmd;
use pom::parser::{seq, sym};
use pom::parser::Parser;

pub(crate) fn for_command<'a>() -> Parser<'a, u8, MastCmd> {
    let condition  = valid_id() - sym(b':');
    (seq(b"for") * space() * valid_id() - space() - seq(b"in") - space() + condition)
        .map(|s| MastCmd::For(s.0,s.1))
}


pub(crate) fn break_command<'a>() -> Parser<'a, u8, MastCmd> {
    seq(b"break").map(|_| MastCmd::Break())
}


pub(crate) fn continue_command<'a>() -> Parser<'a, u8, MastCmd> {
    seq(b"continue").map(|_| MastCmd::Continue())
}

pub(crate) fn end_for_command<'a>() -> Parser<'a, u8, MastCmd> {
    seq(b"next") * space() * valid_id().map(|s| MastCmd::Next(s))
}


#[cfg(test)]
mod tests {
    #[test]
    fn for_test() {
        // One sapce
        let jump = super::for_command().parse(b"for x in d:");
        let _label: String = String::from("x");
        let _cond: String = String::from("d");
        assert!(matches!(jump, Ok(super::MastCmd::For(_label,_cond ))));
       
    }
    #[test]
    fn break_test() {
        let jump = super::break_command().parse(b"break");
        assert!(matches!(jump, Ok(super::MastCmd::Break())));
    }
    #[test]
    fn continue_test() {
        let jump = super::continue_command().parse(b"continue");
        assert!(matches!(jump, Ok(super::MastCmd::Continue())));
    }
    #[test]
    fn end_for_test() {
        let jump = super::end_for_command().parse(b"next x");
        let _label: String = String::from("x");
        assert!(matches!(jump, Ok(super::MastCmd::Next(_label))));
    }
}
