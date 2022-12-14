use pom::char_class::{digit};
use pom::parser::{call, end, is_a, list, seq, sym, Parser};

use std::collections::HashMap;
use std::str::{self, FromStr};

mod json;
use self::json::{JsonValue, value};
mod utils;
use self::utils::space;
mod jump;
use self::jump::jump;
mod identifier;
use self::identifier::valid_id;
mod label;
use self::label::label;
mod if_command;
use self::if_command::{if_command, elif_command, else_command, end_if_command};
mod match_command;
use self::match_command::{match_command, case_command, default_case_command, end_match_command};
mod for_command;
use self::for_command::{for_command, break_command, continue_command, end_for_command};


#[derive(Debug, PartialEq)]
pub enum MastCmd {
	Assign(bool, String, JsonValue),
    Jump(String),
    Push(String),
    Pop(),
    End(),
    PopPush(String),
    PopJump(String),
    If(String),
    ElIf(String),
    Else(),
    EndIf(),

    Match(String),
    Case(JsonValue),
    DefaultCase(),
    EndMatch(),
    For(String, String),
    Break(),
    Continue(),
    Next(String),
    //Label(String, Vec<MastCmd>),

    Parallel(Option<String>,String, Option<JsonValue>),
    CancelName(String),
    AwaitName(String),
    AwaitCondition(String),// TODO
    AwaitParallel(String, Option<JsonValue>),
    AwaitEnd(),

    EventStart(String),
    EventEnd(),
    Delay(String, u32, u32),
    TimeoutLabel(),

    Log(Option<String>,String,Option<String>)
}




fn comment<'a>() -> Parser<'a, u8, ()> {
	sym(b'#').discard() - !sym(b'\n').discard()
}





fn parallel<'a>() -> Parser<'a, u8, MastCmd> {
    let assign = valid_id().opt();
    let id = space().opt() * seq(b"=>") * space().opt() * valid_id();
    let value =  (space().opt() * value()).opt();
    ( assign + id +value).map(|(s,v)| MastCmd::Parallel(s.0,s.1, v))
}


// Parser Combinators	Description
// p | q	Match p or q, return result of the first success.
// p + q	Match p and q, if both succeed return a pair of results.
// p - q	Match p and q, if both succeed return result of p.
// p * q	Match p and q, if both succeed return result of q.
// p >> q	Parse p and get result P, then parse q and return result of q(P).
// -p	Success when p succeeds, doesn't consume input.
// !p	Success when p fails, doesn't consume input.

fn await_parallel<'a>() -> Parser<'a, u8, MastCmd> {
    let id = space().opt() * seq(b"=>") * space().opt() * valid_id();
    let value =  (space().opt() * value()).opt();
    ( seq(b"await") * space() * id +value).map(|(s,v)| MastCmd::AwaitParallel(s, v))
}

fn await_name<'a>() -> Parser<'a, u8, MastCmd> {
    seq(b"await") * space().opt() * valid_id().map(|s| MastCmd::AwaitName(s))
}

fn await_end<'a>() -> Parser<'a, u8, MastCmd> {
    seq(b"end_await").map(|_| MastCmd::AwaitEnd())
}

fn cancel_name<'a>() -> Parser<'a, u8, MastCmd> {
    seq(b"cancel") * space().opt() * valid_id().map(|s| MastCmd::CancelName(s))
}

fn event_name<'a>() -> Parser<'a, u8, MastCmd> {
    seq(b"event") * space() * valid_id().map(|s| MastCmd::EventStart(s)) - sym(b':')
}

fn event_end<'a>() -> Parser<'a, u8, MastCmd> {
    seq(b"end_event").map(|_| MastCmd::EventEnd())
}


fn seconds<'a>() -> Parser<'a, u8, u32> {
    is_a(digit).repeat(0..).collect()
        .convert(str::from_utf8)
        .convert(u32::from_str) - seq(b"s")
}

fn minutes<'a>() -> Parser<'a, u8, u32> {
    is_a(digit).repeat(0..).collect()
        .convert(str::from_utf8)
        .convert(u32::from_str) - seq(b"m")
}


fn duration<'a>() -> Parser<'a, u8, (u32,u32)> {
    (space().opt() * seconds() - space().opt() + minutes() ).map(|s| (s.1,s.0))
    | (space().opt() * minutes() - space().opt() + seconds() ).map(|s| (s.0,s.1))
    | (space().opt() * minutes()).map(|m| (m,0))
    | (space().opt() * seconds() ).map(|s| (0,s))
}

fn delay<'a>() -> Parser<'a, u8, MastCmd> {
    (seq(b"delay")* space() * valid_id() + duration()).map(|(clock,(m,s))| MastCmd::Delay(clock,m,s))
}

fn timeout<'a>() -> Parser<'a, u8, MastCmd> {
    (seq(b"timeout") * space() * valid_id() + duration()).map(|(clock,(m,s))| MastCmd::Delay(clock,m,s))
}

fn timeout_label<'a>() -> Parser<'a, u8, MastCmd> {
    seq(b"timeout:").map(|_| MastCmd::TimeoutLabel())
}



fn variable_def<'a>() -> Parser<'a, u8, MastCmd> {
    let shared = (seq(b"shared")+ space()).opt().map(|s| s.is_some());
    let valid_id = valid_id();
    let assign = shared  + valid_id - sym(b' ') - sym(b'=') + space() *  value();
    assign.map(|(shared, j)| MastCmd::Assign(shared.0, shared.1, j))
  }

fn mast_command<'a>() -> Parser<'a, u8, MastCmd> {
    space().opt() *
	(variable_def().map(|cmd| cmd)
        | jump().map(|cmd| cmd)
        | await_parallel().map(|cmd| cmd)
        | await_name().map(|cmd| cmd)
        | cancel_name().map(|cmd| cmd)
        | parallel().map(|cmd| cmd)
        | delay().map(|cmd| cmd)
        | if_command().map(|cmd| cmd)
        | elif_command().map(|cmd| cmd)
        | else_command().map(|cmd| cmd)
        | end_if_command().map(|cmd| cmd)
        | match_command().map(|cmd| cmd)
        | default_case_command().map(|cmd| cmd)
        | case_command().map(|cmd| cmd)
        | end_match_command().map(|cmd| cmd)
        | for_command().map(|cmd| cmd)
        | break_command().map(|cmd| cmd)
        | continue_command().map(|cmd| cmd)
        | end_for_command().map(|cmd| cmd)
  ) - comment().opt()
}

// pub fn mast_commands<'a>() -> Parser<'a, u8, Vec<MastCmd>> {
// 	let elems = list(call(mast_command), space());
// 	elems - end()
// }

pub struct Label {
    label : String,
    cmds : Vec<MastCmd>
}

pub fn label_block<'a>() -> Parser<'a, u8, Label> {
    let label = space().opt() * label();
    let elems = list(call(mast_command), space());
    (label + elems).map(|s| Label{label: s.0, cmds: s.1})
}
pub fn main_label_block<'a>() -> Parser<'a, u8, Label> {
    let elems = list(call(mast_command), space());
    (elems).map(|s| Label{label: String::from("main"), cmds:s})
}
#[derive(Debug)]
pub struct MastDoc {
    labels : HashMap<String, Vec<MastCmd>>
}

fn to_mast_doc<'a>(main: Label, label_blocks: Vec<Label>) -> MastDoc {
    let mut labels :HashMap<String, Vec<MastCmd>> = label_blocks
        .into_iter()
        .map(|label| (label.label, label.cmds)).collect();
    labels.insert(main.label, main.cmds);

    MastDoc {
        labels
    }
}

pub fn mast_doc<'a>() -> Parser<'a, u8, MastDoc> {
    let main = space().opt() * main_label_block();
    let labels = list(call(label_block), space());
    (main + labels).map(|s| to_mast_doc(s.0,s.1))
}

