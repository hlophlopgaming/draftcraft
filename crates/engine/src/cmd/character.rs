//! Characters: the cast list with cue counts, and renaming a character everywhere (cues keep
//! their extensions; whole-word mentions in the text and card summaries follow; a read-aloud
//! voice moves with the name).

use draftcraft_doc::{Element, Pos, voices::character_key};
use serde_json::{Value, json};

use super::edit::find_in;
use crate::{CmdError, CmdResult, CommandSpec, Session, p};

/// Longest new name accepted (characters).
const MAX_NAME_CHARS: usize = 120;
/// Most mentions replaced in one paragraph or summary.
const MAX_HITS: usize = 10_000;

fn has_characters(s: &Session) -> Option<&'static str> {
    (!s.script.paras.iter().any(|p| p.element == Element::Character && !character_key(&p.text).is_empty())).then_some("the script has no characters")
}

pub fn specs() -> Vec<CommandSpec> {
    vec![
        CommandSpec::new("character.list", "Characters", "Tools › Characters", list).pure(),
        CommandSpec::new("character.rename", "Rename Character", "Tools › Characters", rename)
            .params(r#"{"from": string, "to": string, "inText": bool (default true: also mentions in the text and card summaries)}"#)
            .when(has_characters),
    ]
}

/// Characters in order of first appearance, with how many cues each has.
fn list(s: &mut Session, _: &Value) -> CmdResult {
    let mut out: Vec<(String, usize)> = Vec::new();
    for p in &s.script.paras {
        if p.element != Element::Character {
            continue;
        }
        let key = character_key(&p.text);
        if key.is_empty() {
            continue;
        }
        match out.iter_mut().find(|(k, _)| *k == key) {
            Some((_, n)) => *n += 1,
            None => out.push((key, 1)),
        }
    }
    Ok(json!({"characters": out.iter().map(|(name, cues)| json!({"name": name, "cues": cues})).collect::<Vec<_>>()}))
}

fn rename(s: &mut Session, v: &Value) -> CmdResult {
    let from = character_key(p::req_str(v, "from")?);
    if from.is_empty() {
        return Err(CmdError::Params("`from` is empty".into()));
    }
    let to = clean_name(p::req_str(v, "to")?)?;
    let in_text = p::bool(v, "inText").unwrap_or(true);

    let mut cue_paras = Vec::new();
    for (i, p) in s.script.paras.iter().enumerate() {
        if p.element == Element::Character && character_key(&p.text) == from {
            cue_paras.push(i);
        }
    }
    if cue_paras.is_empty() {
        return Err(CmdError::Params(format!("no character named {from}")));
    }
    for &i in &cue_paras {
        let Some((a, b)) = s.script.para(i).and_then(|p| name_range(&p.text)) else { continue };
        s.script.delete_range(Pos::new(i, a), Pos::new(i, b));
        s.script.insert_text(Pos::new(i, a), &to)?;
    }

    let mut mentions = 0usize;
    if in_text {
        for i in 0..s.script.paras.len() {
            let Some(para) = s.script.para(i) else { break };
            if para.element == Element::Character {
                continue;
            }
            let hits: Vec<(usize, usize, String)> = word_matches(&para.text, &from)
                .into_iter()
                .filter_map(|(a, b)| para.text.get(a..b).map(|found| (a, b, cased_like(found, &to))))
                .collect();
            // Back to front, so earlier offsets stay valid.
            for (a, b, with) in hits.into_iter().rev() {
                s.script.delete_range(Pos::new(i, a), Pos::new(i, b));
                s.script.insert_text(Pos::new(i, a), &with)?;
                mentions += 1;
            }
            if let Some(para) = s.script.paras.get_mut(i)
                && !para.summary.is_empty()
            {
                let (text, n) = replace_words(&para.summary, &from, &to);
                para.summary = text;
                mentions += n;
            }
        }
    }

    let to_key = character_key(&to);
    let cast = &mut s.script.settings.voices.cast;
    if to_key != from
        && let Some(voice) = cast.remove(&from)
    {
        cast.entry(to_key).or_insert(voice);
    }

    Ok(json!({"from": from, "to": to, "cues": cue_paras.len(), "mentions": mentions}))
}

/// The new name as stored: one line, trimmed, at most 120 characters, no cue-extension marks.
fn clean_name(raw: &str) -> Result<String, CmdError> {
    let t: String = draftcraft_doc::sanitize(raw).chars().take(MAX_NAME_CHARS).collect();
    let t = t.trim();
    if t.is_empty() {
        return Err(CmdError::Params("the new name is empty".into()));
    }
    if t.contains(['(', ')', '^']) {
        return Err(CmdError::Params("a name can't contain ( ) or ^: they mark cue extensions".into()));
    }
    Ok(t.to_string())
}

/// Byte range of the name in a cue: `"  NELL (V.O.)"` → `(2, 6)`, `"TEO ^"` → `(0, 3)`.
fn name_range(cue: &str) -> Option<(usize, usize)> {
    let base = cue.get(..cue.find('(').unwrap_or(cue.len()))?;
    let end = base.trim_end().trim_end_matches('^').trim_end().len();
    let start = base.len().saturating_sub(base.trim_start().len());
    (start < end).then_some((start, end))
}

fn is_word_char(c: char) -> bool {
    c.is_alphanumeric() || c == '_'
}

/// Whole-word, case-insensitive matches of `name` in `text` (byte ranges, in order).
fn word_matches(text: &str, name: &str) -> Vec<(usize, usize)> {
    let mut out = Vec::new();
    let mut at = 0;
    while let Some((a, b)) = find_in(text, name, at, false) {
        let before = text.get(..a).and_then(|t| t.chars().next_back());
        let after = text.get(b..).and_then(|t| t.chars().next());
        if !before.is_some_and(is_word_char) && !after.is_some_and(is_word_char) {
            out.push((a, b));
            if out.len() >= MAX_HITS {
                break;
            }
        }
        if b <= at {
            break;
        }
        at = b;
    }
    out
}

/// The new name written the way the old one was: `NELL` → `MARA`; `Nell` → `Mara` (a name
/// given in capitals is title-cased, any other spelling is kept as typed).
fn cased_like(found: &str, to: &str) -> String {
    if !found.chars().any(char::is_lowercase) {
        return to.to_uppercase();
    }
    if to.chars().any(char::is_lowercase) { to.to_string() } else { title_case(to) }
}

/// `"MARA DE-LUCA"` → `"Mara De-Luca"`, `"O'BRIEN"` → `"O'Brien"`.
fn title_case(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    let mut start = true;
    for c in s.chars() {
        if start {
            out.extend(c.to_uppercase());
        } else {
            out.extend(c.to_lowercase());
        }
        start = !c.is_alphanumeric();
    }
    out
}

/// Replace whole-word mentions in plain text (card summaries); returns the text and the count.
fn replace_words(text: &str, name: &str, to: &str) -> (String, usize) {
    let hits = word_matches(text, name);
    if hits.is_empty() {
        return (text.to_string(), 0);
    }
    let mut out = String::with_capacity(text.len());
    let mut at = 0;
    for &(a, b) in &hits {
        out.push_str(text.get(at..a).unwrap_or(""));
        out.push_str(&cased_like(text.get(a..b).unwrap_or(""), to));
        at = b;
    }
    out.push_str(text.get(at..).unwrap_or(""));
    (draftcraft_doc::sanitize(&out), hits.len())
}

#[cfg(test)]
mod tests {
    use draftcraft_doc::{Element, Para, Script};
    use serde_json::json;

    use crate::Session;

    fn session() -> Session {
        let mut heading = Para::new(Element::SceneHeading, "INT. NELL'S KITCHEN - NIGHT");
        heading.summary = "Nell can't sleep.".into();
        let paras = vec![
            heading,
            Para::new(Element::Action, "Nell pours tea. NELL ORTIZ never sleeps. Nellie waves."),
            Para::new(Element::Character, "NELL (V.O.)"),
            Para::new(Element::Dialogue, "Teo, it's Nell."),
            Para::new(Element::Character, "TEO"),
            Para::new(Element::Dialogue, "Hi, Nell!"),
            Para::new(Element::Character, "NELL (CONT'D)"),
            Para::new(Element::Dialogue, "Again."),
        ];
        Session::new(Script::from_paras(paras).unwrap())
    }

    fn text(s: &Session, i: usize) -> &str {
        s.script.paras[i].text.as_str()
    }

    #[test]
    fn list_counts_cues() {
        let mut s = session();
        let v = s.run("character.list", &json!({})).unwrap();
        assert_eq!(v["characters"], json!([{"name": "NELL", "cues": 2}, {"name": "TEO", "cues": 1}]));
    }

    #[test]
    fn rename_everywhere() {
        let mut s = session();
        s.script.settings.voices.cast.insert("NELL".into(), "Samantha".into());
        let v = s.run("character.rename", &json!({"from": "nell", "to": "MARA"})).unwrap();
        assert_eq!(v["cues"], 2);
        assert_eq!(v["mentions"], 6);
        assert_eq!(text(&s, 2), "MARA (V.O.)");
        assert_eq!(text(&s, 6), "MARA (CONT'D)");
        assert_eq!(text(&s, 0), "INT. MARA'S KITCHEN - NIGHT");
        assert_eq!(text(&s, 1), "Mara pours tea. MARA ORTIZ never sleeps. Nellie waves.");
        assert_eq!(text(&s, 3), "Teo, it's Mara.");
        assert_eq!(text(&s, 5), "Hi, Mara!");
        assert_eq!(s.script.paras[0].summary, "Mara can't sleep.");
        assert_eq!(s.script.settings.voices.cast.get("MARA").map(String::as_str), Some("Samantha"));
        assert!(!s.script.settings.voices.cast.contains_key("NELL"));
        // One undo step brings everything back.
        s.run("edit.undo", &json!({})).unwrap();
        assert_eq!(text(&s, 2), "NELL (V.O.)");
        assert_eq!(text(&s, 1), "Nell pours tea. NELL ORTIZ never sleeps. Nellie waves.");
    }

    #[test]
    fn rename_cues_only_and_keep_typed_spelling() {
        let mut s = session();
        s.run("character.rename", &json!({"from": "NELL", "to": "McKenzie", "inText": false})).unwrap();
        assert_eq!(text(&s, 2), "MCKENZIE (V.O.)");
        assert_eq!(text(&s, 3), "Teo, it's Nell.");
        s.run("character.rename", &json!({"from": "TEO", "to": "Сава"})).unwrap();
        assert_eq!(text(&s, 4), "САВА");
        assert_eq!(text(&s, 3), "Сава, it's Nell.");
    }

    #[test]
    fn rename_rejects_bad_names() {
        let mut s = session();
        assert!(s.run("character.rename", &json!({"from": "NOBODY", "to": "X"})).is_err());
        assert!(s.run("character.rename", &json!({"from": "NELL", "to": "  "})).is_err());
        assert!(s.run("character.rename", &json!({"from": "NELL", "to": "MARA (V.O.)"})).is_err());
        assert!(s.run("character.rename", &json!({"from": "", "to": "MARA"})).is_err());
        assert_eq!(text(&s, 2), "NELL (V.O.)");
        assert!(Session::new(Script::new()).run("character.rename", &json!({"from": "A", "to": "B"})).is_err());
    }

    #[test]
    fn helpers() {
        assert_eq!(super::name_range("  NELL (V.O.)"), Some((2, 6)));
        assert_eq!(super::name_range("TEO ^"), Some((0, 3)));
        assert_eq!(super::name_range("(V.O.)"), None);
        assert_eq!(super::title_case("O'BRIEN DE-LUCA"), "O'Brien De-Luca");
        assert_eq!(super::word_matches("Nell, Nellie, nell", "NELL"), vec![(0, 4), (14, 18)]);
    }
}
