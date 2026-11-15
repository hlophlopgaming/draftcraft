//! Format Assistant: finds slips in the screenplay format (a cue with no dialogue, two
//! parentheticals in a row, an empty scene…) and steps through them. It only points; the writer
//! decides what to change.

use draftcraft_doc::{Element, Para, Pos};
use serde_json::{Value, json};

use crate::{CmdError, CmdResult, CommandSpec, Selection, Session};

/// Most issues reported at once.
const MAX_ISSUES: usize = 5_000;
/// Characters of the paragraph's text quoted with an issue.
const QUOTE_CHARS: usize = 80;

/// What the assistant checks.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Kind {
    /// A character cue with no dialogue under it.
    CueWithoutDialogue,
    /// Dialogue under an empty character cue.
    EmptyCue,
    /// Dialogue or a parenthetical with no character cue above it.
    DialogueWithoutCue,
    /// A parenthetical straight after another one.
    ParentheticalsInARow,
    /// A parenthetical that ends a speech, with no dialogue after it.
    ParentheticalLast,
    /// A scene heading with nothing under it before the next scene.
    EmptyScene,
}

impl Kind {
    /// Every check, in the order they are described.
    pub const ALL: [Kind; 6] =
        [Kind::CueWithoutDialogue, Kind::EmptyCue, Kind::DialogueWithoutCue, Kind::ParentheticalsInARow, Kind::ParentheticalLast, Kind::EmptyScene];

    /// The stable id used by commands.
    pub const fn id(self) -> &'static str {
        match self {
            Kind::CueWithoutDialogue => "cueWithoutDialogue",
            Kind::EmptyCue => "emptyCue",
            Kind::DialogueWithoutCue => "dialogueWithoutCue",
            Kind::ParentheticalsInARow => "parentheticalsInARow",
            Kind::ParentheticalLast => "parentheticalLast",
            Kind::EmptyScene => "emptyScene",
        }
    }

    /// What is wrong, in a sentence.
    pub const fn message(self) -> &'static str {
        match self {
            Kind::CueWithoutDialogue => "A character cue with no dialogue under it.",
            Kind::EmptyCue => "Dialogue under an empty character cue: who is speaking?",
            Kind::DialogueWithoutCue => "Dialogue with no character cue above it.",
            Kind::ParentheticalsInARow => "Two parentheticals in a row: join them, or put dialogue between them.",
            Kind::ParentheticalLast => "A parenthetical ends the speech: move it into the action, or add dialogue after it.",
            Kind::EmptyScene => "An empty scene: nothing under the heading before the next scene.",
        }
    }

    fn parse(s: &str) -> Option<Kind> {
        Kind::ALL.into_iter().find(|k| k.id().eq_ignore_ascii_case(s.trim()))
    }
}

/// One slip: the paragraph it is in and what it is.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Issue {
    pub para: usize,
    pub kind: Kind,
}

fn blank(p: &Para) -> bool {
    p.text.trim().is_empty()
}

fn in_speech(e: Element) -> bool {
    matches!(e, Element::Parenthetical | Element::Dialogue)
}

/// The paragraphs right after `i` whose element passes `keep` (up to the first that doesn't).
fn run_after(paras: &[Para], i: usize, keep: impl Fn(Element) -> bool) -> &[Para] {
    let rest = paras.get(i.saturating_add(1)..).unwrap_or_default();
    let n = rest.iter().take_while(|q| keep(q.element)).count();
    rest.get(..n).unwrap_or_default()
}

/// Every slip in the paragraphs, in script order (at most [`MAX_ISSUES`]).
pub fn check(paras: &[Para]) -> Vec<Issue> {
    let mut out = Vec::new();
    for (i, p) in paras.iter().enumerate() {
        let prev = i.checked_sub(1).and_then(|j| paras.get(j));
        let next = paras.get(i + 1);
        let mut push = |kind| out.push(Issue { para: i, kind });
        match p.element {
            Element::Character => {
                let speech = run_after(paras, i, in_speech);
                let has_dialogue = speech.iter().any(|q| q.element == Element::Dialogue && !blank(q));
                let has_speech = speech.iter().any(|q| !blank(q));
                if blank(p) {
                    if has_speech {
                        push(Kind::EmptyCue);
                    }
                } else if !has_dialogue {
                    push(Kind::CueWithoutDialogue);
                }
            }
            Element::Parenthetical | Element::Dialogue if !blank(p) => {
                if prev.is_none_or(|q| q.element != Element::Character && !in_speech(q.element)) {
                    push(Kind::DialogueWithoutCue);
                }
                if p.element == Element::Parenthetical {
                    if prev.is_some_and(|q| q.element == Element::Parenthetical && !blank(q)) {
                        push(Kind::ParentheticalsInARow);
                    }
                    if prev.is_some_and(|q| q.element == Element::Dialogue) && next.is_none_or(|q| !in_speech(q.element) || blank(q)) {
                        push(Kind::ParentheticalLast);
                    }
                }
            }
            Element::SceneHeading if !blank(p) && !p.omitted && run_after(paras, i, |e| e != Element::SceneHeading).iter().all(blank) => {
                push(Kind::EmptyScene);
            }
            _ => {}
        }
        if out.len() >= MAX_ISSUES {
            out.truncate(MAX_ISSUES);
            break;
        }
    }
    out
}

pub fn specs() -> Vec<CommandSpec> {
    const KINDS: &str = r#"{"kinds": [string] (optional: only these checks — cueWithoutDialogue, emptyCue, dialogueWithoutCue, parentheticalsInARow, parentheticalLast, emptyScene)}"#;
    vec![
        CommandSpec::new("assist.check", "Format Assistant", "Tools › Format Assistant", check_cmd).params(KINDS).pure(),
        CommandSpec::new("assist.next", "Next Format Issue", "Tools › Format Assistant", next).params(KINDS).pure(),
        CommandSpec::new("assist.previous", "Previous Format Issue", "Tools › Format Assistant", previous).params(KINDS).pure(),
    ]
}

/// The checks asked for in `kinds` (all of them when it is missing).
fn kinds(v: &Value) -> Result<Vec<Kind>, CmdError> {
    let Some(list) = v.get("kinds") else { return Ok(Kind::ALL.to_vec()) };
    let Some(list) = list.as_array() else { return Err(CmdError::Params("`kinds` must be a list of check ids".into())) };
    let mut out = Vec::new();
    for k in list.iter().take(Kind::ALL.len() * 4) {
        let id = k.as_str().unwrap_or_default();
        let kind = Kind::parse(id).ok_or_else(|| CmdError::Params(format!("unknown check `{id}`")))?;
        if !out.contains(&kind) {
            out.push(kind);
        }
    }
    Ok(out)
}

fn issues(s: &Session, v: &Value) -> Result<Vec<Issue>, CmdError> {
    let wanted = kinds(v)?;
    Ok(check(&s.script.paras).into_iter().filter(|i| wanted.contains(&i.kind)).collect())
}

fn issue_json(s: &Session, index: usize, i: Issue) -> Value {
    let text: String = s.script.para(i.para).map(|p| p.text.chars().take(QUOTE_CHARS).collect()).unwrap_or_default();
    json!({
        "index": index,
        "para": i.para,
        "kind": i.kind.id(),
        "message": i.kind.message(),
        "scene": s.script.scene_of(i.para),
        "text": text,
    })
}

fn check_cmd(s: &mut Session, v: &Value) -> CmdResult {
    let found = issues(s, v)?;
    let truncated = found.len() >= MAX_ISSUES;
    let list: Vec<Value> = found.iter().enumerate().map(|(n, i)| issue_json(s, n, *i)).collect();
    Ok(json!({"count": list.len(), "truncated": truncated, "issues": list}))
}

/// Select the issue's paragraph and describe it.
fn select(s: &mut Session, index: usize, i: Issue) -> CmdResult {
    let len = s.script.para(i.para).map_or(0, |p| p.text.len());
    s.sel = Selection { anchor: s.script.clamp(Pos::new(i.para, 0)), focus: s.script.clamp(Pos::new(i.para, len)) };
    Ok(issue_json(s, index, i))
}

fn no_issues() -> CmdError {
    CmdError::Disabled("the script has no format issues".into())
}

/// The first issue in a paragraph after the caret's (wrapping to the top).
fn next(s: &mut Session, v: &Value) -> CmdResult {
    let found = issues(s, v)?;
    let at = s.sel.ordered().0.para;
    let index = found.iter().position(|i| i.para > at).or((!found.is_empty()).then_some(0)).ok_or_else(no_issues)?;
    let issue = *found.get(index).ok_or_else(no_issues)?;
    select(s, index, issue)
}

/// The last issue in a paragraph before the caret's (wrapping to the bottom).
fn previous(s: &mut Session, v: &Value) -> CmdResult {
    let found = issues(s, v)?;
    let at = s.sel.ordered().0.para;
    let index = found.iter().rposition(|i| i.para < at).or(found.len().checked_sub(1)).ok_or_else(no_issues)?;
    let issue = *found.get(index).ok_or_else(no_issues)?;
    select(s, index, issue)
}

#[cfg(test)]
mod tests {
    use draftcraft_doc::{Element, Para, Pos, Script};
    use serde_json::json;

    use crate::{Selection, Session};

    fn session() -> Session {
        let paras = vec![
            Para::new(Element::SceneHeading, "INT. KITCHEN - NIGHT"), // 0
            Para::new(Element::Action, "Nell pours tea."),            // 1
            Para::new(Element::Character, "NELL"),                    // 2: no dialogue
            Para::new(Element::Character, "TEO"),                     // 3
            Para::new(Element::Parenthetical, "(quietly)"),           // 4
            Para::new(Element::Parenthetical, "(then louder)"),       // 5: two in a row
            Para::new(Element::Dialogue, "Nell?"),                    // 6
            Para::new(Element::Parenthetical, "(beat)"),              // 7: ends the speech
            Para::new(Element::SceneHeading, "EXT. YARD - DAY"),      // 8: empty scene
            Para::new(Element::Action, ""),                           // 9
            Para::new(Element::SceneHeading, "INT. HALL - DAY"),      // 10
            Para::new(Element::Dialogue, "Hello?"),                   // 11: no cue
            Para::new(Element::Character, ""),                        // 12: empty cue
            Para::new(Element::Dialogue, "Who's there?"),             // 13
        ];
        Session::new(Script::from_paras(paras).unwrap())
    }

    fn found(v: &serde_json::Value) -> Vec<(u64, String)> {
        v["issues"].as_array().unwrap().iter().map(|i| (i["para"].as_u64().unwrap(), i["kind"].as_str().unwrap().to_string())).collect()
    }

    #[test]
    fn finds_every_kind() {
        let mut s = session();
        let v = s.run("assist.check", &json!({})).unwrap();
        let want: Vec<(u64, String)> = [
            (2, "cueWithoutDialogue"),
            (5, "parentheticalsInARow"),
            (7, "parentheticalLast"),
            (8, "emptyScene"),
            (11, "dialogueWithoutCue"),
            (12, "emptyCue"),
        ]
        .into_iter()
        .map(|(p, k)| (p, k.to_string()))
        .collect();
        assert_eq!(found(&v), want);
        assert_eq!(v["count"], 6);
        assert_eq!(v["issues"][0]["text"], "NELL");
        assert_eq!(v["issues"][3]["scene"], 1);
    }

    #[test]
    fn only_some_kinds() {
        let mut s = session();
        let v = s.run("assist.check", &json!({"kinds": ["emptyScene", "EMPTYCUE"]})).unwrap();
        assert_eq!(found(&v), vec![(8, "emptyScene".to_string()), (12, "emptyCue".to_string())]);
        assert!(s.run("assist.check", &json!({"kinds": ["nope"]})).is_err());
        assert!(s.run("assist.check", &json!({"kinds": "emptyScene"})).is_err());
    }

    #[test]
    fn clean_script_has_no_issues() {
        let paras = vec![
            Para::new(Element::SceneHeading, "INT. KITCHEN - NIGHT"),
            Para::new(Element::Action, "Nell pours tea."),
            Para::new(Element::Character, "NELL"),
            Para::new(Element::Parenthetical, "(quietly)"),
            Para::new(Element::Dialogue, "Teo?"),
            Para::new(Element::Character, "TEO"),
            Para::new(Element::Dialogue, "Here."),
            Para::new(Element::Parenthetical, "(beat)"),
            Para::new(Element::Dialogue, "Always."),
            Para::new(Element::Action, ""),
        ];
        let mut s = Session::new(Script::from_paras(paras).unwrap());
        assert_eq!(s.run("assist.check", &json!({})).unwrap()["count"], 0);
        assert!(s.run("assist.next", &json!({})).is_err());
        // A new, blank script is not an empty scene yet.
        assert_eq!(Session::new(Script::new()).run("assist.check", &json!({})).unwrap()["count"], 0);
    }

    #[test]
    fn next_and_previous_wrap() {
        let mut s = session();
        s.sel = Selection::caret(Pos::new(6, 1));
        let v = s.run("assist.next", &json!({})).unwrap();
        assert_eq!((v["para"].as_u64(), v["index"].as_u64()), (Some(7), Some(2)));
        assert_eq!(s.sel, Selection { anchor: Pos::new(7, 0), focus: Pos::new(7, "(beat)".len()) });
        s.sel = Selection::caret(Pos::new(13, 0));
        assert_eq!(s.run("assist.next", &json!({})).unwrap()["para"], 2);
        assert_eq!(s.run("assist.previous", &json!({})).unwrap()["para"], 12);
        s.sel = Selection::caret(Pos::new(0, 0));
        assert_eq!(s.run("assist.previous", &json!({})).unwrap()["para"], 12);
        assert_eq!(s.run("assist.next", &json!({"kinds": ["emptyScene"]})).unwrap()["para"], 8);
        // Looking around changes nothing.
        assert!(!s.dirty);
    }
}
