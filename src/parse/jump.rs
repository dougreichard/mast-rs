use super::utils::space;
use super::valid_id;
use super::MastCmd;
use pom::parser::seq;
use pom::parser::Parser;

pub(crate) fn jump<'a>() -> Parser<'a, u8, MastCmd> {
    seq(b"->") * space().opt() * seq(b"END").map(|s| MastCmd::End())
        | seq(b"->") * space() * valid_id().map(|s| MastCmd::Jump(s))
        | seq(b"->>") * space().opt() * valid_id().map(|s| MastCmd::Push(s))
        | seq(b"<<->>") * space().opt() * valid_id().map(|s| MastCmd::PopPush(s))
        | seq(b"<<->") * space().opt() * valid_id().map(|s| MastCmd::PopJump(s))
        | seq(b"<<-").map(|_| MastCmd::Pop())
}

#[cfg(test)]
mod tests {


    #[test]
    fn jump_test() {
        // One sapce
        let jump = super::jump().parse(b"-> MyLabel");
        let _label: String = String::from("MyLabel");
        assert!(matches!(jump, Ok(super::MastCmd::Jump(_label))));
        // multiple spaces
        let jump = super::jump().parse(b"->      MyLabel");
        let _label: String = String::from("MyLabel");
        assert!(matches!(jump, Ok(super::MastCmd::Jump(_label))));
        // No Spaces
        let jump = super::jump().parse(b"->MyLabel");
        let _label: String = String::from("MyLabel");
        assert!(matches!(jump, Ok(super::MastCmd::Jump(_label))));
    }
    #[test]
    fn push_test() {
        // One sapce
        let jump = super::jump().parse(b"->> MyLabel");
        let _label: String = String::from("MyLabel");
        assert!(matches!(jump, Ok(super::MastCmd::Push(_label))));
        // multiple spaces
        let jump = super::jump().parse(b"->>      MyLabel");
        let _label: String = String::from("MyLabel");
        assert!(matches!(jump, Ok(super::MastCmd::Push(_label))));
        // No Spaces
        let jump = super::jump().parse(b"->>MyLabel");
        let _label: String = String::from("MyLabel");
        assert!(matches!(jump, Ok(super::MastCmd::Push(_label))));
    }
    #[test]
    fn pop_test() {
        // One sapce
        let jump = super::jump().parse(b"<<-");
        assert!(matches!(jump, Ok(super::MastCmd::Pop())));
    }
    #[test]
    fn end_test() {
        // One sapce
        let jump = super::jump().parse(b"->    END");
        assert!(matches!(jump, Ok(super::MastCmd::End())));
        let jump = super::jump().parse(b"->END");
        assert!(matches!(jump, Ok(super::MastCmd::End())));
        let jump = super::jump().parse(b"-> END");
        assert!(matches!(jump, Ok(super::MastCmd::End())));
    }
    #[test]
    fn pup_jump_test() {
        // One sapce
        let jump = super::jump().parse(b"<<->       jump_pop");
        let _label: String = String::from("jump_pop");
        assert!(matches!(jump, Ok(super::MastCmd::PopJump(_label))));
        let jump = super::jump().parse(b"<<->jump_pop");
        assert!(matches!(jump, Ok(super::MastCmd::PopJump(_label))));
        let jump = super::jump().parse(b"<<-> jump_pop");
        assert!(matches!(jump, Ok(super::MastCmd::PopJump(_label))));
    }
    #[test]
    fn pup_push_test() {
        // One sapce
        let jump = super::jump().parse(b"<<->>       jump_push");
        let _label: String = String::from("jump_push");
        assert!(matches!(jump, Ok(super::MastCmd::PopPush(_label))));
        let jump = super::jump().parse(b"<<->jump_push");
        assert!(matches!(jump, Ok(super::MastCmd::PopJump(_label))));
        let jump = super::jump().parse(b"<<-> jump_push");
        assert!(matches!(jump, Ok(super::MastCmd::PopJump(_label))));
    }
}
