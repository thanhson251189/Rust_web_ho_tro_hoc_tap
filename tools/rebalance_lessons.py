from pathlib import Path
import re
from collections import Counter, defaultdict

ROOT = Path(__file__).resolve().parents[1]
text = (ROOT / "src/lessons.rs").read_text(encoding="utf-8")
pat = re.compile(
    r'L!\((\d+),\s*Subject::(\w+),\s*"([^"]*)",\s*"([^"]*)",\s*"([^"]*)",\s*(Picture::\w+(?:\(\d+\))?),\s*\[([^\]]*)\],\s*(\d+)\)'
)
lessons = []
for m in pat.finditer(text):
    lid, sub, unit, title, prompt, pic, ch, ok = m.groups()
    choices = re.findall(r'"([^"]*)"', ch)
    assert len(choices) == 4, (lid, choices)
    lessons.append(
        {
            "id": int(lid),
            "sub": sub,
            "unit": unit,
            "title": title,
            "prompt": prompt,
            "pic": pic,
            "choices": choices,
            "ok": int(ok),
        }
    )
assert len(lessons) == 126, len(lessons)

for L in lessons:
    if L["id"] == 30:
        L["title"] = "Bài 15. Trái — phải"
        L["prompt"] = (
            "Trên thước, số 2 nằm bên trái số 3. Số 4 nằm bên nào so với số 3?"
        )
        L["choices"] = ["trên", "phải", "dưới", "giữa"]
        L["ok"] = 1

by_sub = defaultdict(list)
for L in lessons:
    by_sub[L["sub"]].append(L)


def unit_num(u: str) -> int:
    m = re.search(r"Unit (\d+)", u)
    return int(m.group(1)) if m else 99


eng = sorted(by_sub["TiengAnh"], key=lambda L: (unit_num(L["unit"]), L["id"]))
assert unit_num(eng[0]["unit"]) == 1
assert unit_num(eng[-1]["unit"]) == 16
for i, L in enumerate(eng):
    L["id"] = 95 + i
by_sub["TiengAnh"] = eng


def place(choices, old_ok, new_ok):
    ans = choices[old_ok]
    others = [c for i, c in enumerate(choices) if i != old_ok]
    out = [None] * 4
    out[new_ok] = ans
    j = 0
    for i in range(4):
        if i == new_ok:
            continue
        out[i] = others[j]
        j += 1
    return out, new_ok


for sub, items in by_sub.items():
    counts = Counter()
    for i, L in enumerate(items):
        target = min(range(4), key=lambda s: (counts[s], s))
        if i % 5 == 4:
            ordered_slots = sorted(range(4), key=lambda s: (counts[s], s))
            target = ordered_slots[1]
        L["choices"], L["ok"] = place(L["choices"], L["ok"], target)
        counts[L["ok"]] += 1
    n = len(items)
    assert len(counts) == 4
    assert max(counts.values()) / n <= 0.40 + 1e-9, (sub, counts)

toan = by_sub["Toan"]
viet = by_sub["TiengViet"]
anh = by_sub["TiengAnh"]
assert [L["id"] for L in toan] == list(range(1, 55))
assert [L["id"] for L in viet] == list(range(55, 95))
assert [L["id"] for L in anh] == list(range(95, 127))
ordered = toan + viet + anh
assert len(ordered) == 126
nums = [unit_num(L["unit"]) for L in anh]
assert nums == sorted(nums)
assert nums[-1] == 16
assert not any(
    "tay phải" in (L["prompt"] + " " + " ".join(L["choices"])) for L in ordered
)

comments = {
    1: "    // Chủ đề 1 — Các số từ 0 đến 10",
    13: "    // Chủ đề 2 — Hình phẳng",
    18: "    // Chủ đề 3 — Cộng trừ phạm vi 10",
    27: "    // Chủ đề 4 — Hình khối",
    32: "    // Chủ đề 5 — Ôn HK1",
    36: "    // Tập 2 — Chủ đề 6 số đến 100",
    41: "    // Chủ đề 7 đo độ dài",
    44: "    // Chủ đề 8 cộng trừ không nhớ phạm vi 100",
    49: "    // Chủ đề 9 giờ và lịch",
    52: "    // Chủ đề 10 ôn cuối năm",
    55: "    // Tiếng Việt KNTT tập 1 — chữ và vần",
    87: "    // Tiếng Việt KNTT tập 2 — đọc hiểu ngắn",
    95: "    // Tiếng Anh 1 Global Success — 16 unit (thứ tự unit tăng dần)",
}
lines = []
for L in ordered:
    if L["id"] in comments:
        lines.append(comments[L["id"]])
    ch = ", ".join(f'"{c}"' for c in L["choices"])
    lines.append(
        f'    L!({L["id"]}, Subject::{L["sub"]}, "{L["unit"]}", "{L["title"]}", "{L["prompt"]}", {L["pic"]}, [{ch}], {L["ok"]}),'
    )
body = "\n".join(lines)

header = r'''#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Subject {
    Toan,
    TiengViet,
    TiengAnh,
}

impl Subject {
    pub fn slug(self) -> &'static str {
        match self {
            Subject::Toan => "toan",
            Subject::TiengViet => "tieng-viet",
            Subject::TiengAnh => "tieng-anh",
        }
    }

    pub fn title(self) -> &'static str {
        match self {
            Subject::Toan => "Toán",
            Subject::TiengViet => "Tiếng Việt",
            Subject::TiengAnh => "Tiếng Anh",
        }
    }

    pub fn hint(self) -> &'static str {
        match self {
            Subject::Toan => "Bám SGK KNTT tập 1–2",
            Subject::TiengViet => "Bám SGK KNTT tập 1–2",
            Subject::TiengAnh => "Global Success 16 unit",
        }
    }

    pub fn parse(slug: &str) -> Option<Self> {
        match slug {
            "toan" => Some(Subject::Toan),
            "tieng-viet" => Some(Subject::TiengViet),
            "tieng-anh" => Some(Subject::TiengAnh),
            _ => None,
        }
    }

    pub fn all() -> [Subject; 3] {
        [Subject::Toan, Subject::TiengViet, Subject::TiengAnh]
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Picture {
    None,
    Stars(u8),
    Apples(u8),
    Dots(u8),
    Blocks(u8),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Lesson {
    pub id: u32,
    pub subject: Subject,
    pub unit: &'static str,
    pub title: &'static str,
    pub prompt: &'static str,
    pub picture: Picture,
    pub choices: [&'static str; 4],
    pub correct: usize,
}

pub fn all_lessons() -> &'static [Lesson] {
    &LESSONS
}

pub fn by_id(id: u32) -> Option<&'static Lesson> {
    LESSONS.iter().find(|lesson| lesson.id == id)
}

pub fn for_subject(subject: Subject) -> Vec<&'static Lesson> {
    LESSONS
        .iter()
        .filter(|lesson| lesson.subject == subject)
        .collect()
}

pub fn next_after(lesson: &Lesson) -> Option<&'static Lesson> {
    let rest: Vec<_> = for_subject(lesson.subject);
    let pos = rest.iter().position(|item| item.id == lesson.id)?;
    rest.get(pos + 1).copied()
}

pub fn last_id(subject: Subject) -> u32 {
    for_subject(subject)
        .last()
        .map(|lesson| lesson.id)
        .unwrap_or(0)
}

macro_rules! L {
    ($id:expr, $sub:expr, $unit:expr, $title:expr, $prompt:expr, $pic:expr, [$a:expr, $b:expr, $c:expr, $d:expr], $ok:expr) => {
        Lesson {
            id: $id,
            subject: $sub,
            unit: $unit,
            title: $title,
            prompt: $prompt,
            picture: $pic,
            choices: [$a, $b, $c, $d],
            correct: $ok,
        }
    };
}

// Original tap-to-answer items aligned to SGK outlines. Do not host textbook pages/art.
#[rustfmt::skip]
const LESSONS: [Lesson; 126] = [
'''

footer = r'''
];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn math_follows_sgk_length() {
        assert_eq!(for_subject(Subject::Toan).len(), 54);
        assert_eq!(for_subject(Subject::TiengViet).len(), 40);
        assert_eq!(for_subject(Subject::TiengAnh).len(), 32);
    }

    #[test]
    fn math_covers_both_volumes() {
        let titles: Vec<_> = for_subject(Subject::Toan)
            .into_iter()
            .map(|lesson| lesson.title)
            .collect();
        assert!(titles.iter().any(|t| t.contains("Bài 1.")));
        assert!(titles.iter().any(|t| t.contains("Bài 21.")));
        assert!(titles.iter().any(|t| t.contains("Bài 40.")));
    }

    #[test]
    fn correct_index_is_in_range() {
        for lesson in all_lessons() {
            assert!(lesson.correct < lesson.choices.len());
        }
    }

    #[test]
    fn correct_slots_are_balanced_per_subject() {
        for subject in Subject::all() {
            let lessons = for_subject(subject);
            let mut counts = [0usize; 4];
            for lesson in &lessons {
                counts[lesson.correct] += 1;
            }
            assert!(
                counts.iter().all(|&c| c > 0),
                "{subject:?} missing a slot: {counts:?}"
            );
            let max_share = *counts.iter().max().unwrap() as f64 / lessons.len() as f64;
            assert!(
                max_share <= 0.40 + f64::EPSILON,
                "{subject:?} slot share {max_share:.2} counts={counts:?}"
            );
        }
    }

    #[test]
    fn left_right_lesson_is_handedness_neutral() {
        let lesson = by_id(30).expect("lesson 30");
        let blob = format!("{} {} {}", lesson.prompt, lesson.title, lesson.choices.join(" "));
        assert!(!blob.contains("tay phải"));
        assert!(!blob.contains("tay trái"));
        assert!(lesson.prompt.contains("trái") || lesson.choices.iter().any(|c| *c == "phải"));
    }

    #[test]
    fn english_units_increase_and_end_at_16() {
        let mut prev = 0u32;
        let mut last = 0u32;
        for lesson in for_subject(Subject::TiengAnh) {
            let n: u32 = lesson
                .unit
                .split_whitespace()
                .nth(1)
                .and_then(|s| s.parse().ok())
                .expect("Unit N");
            assert!(n >= prev, "unit regress {prev} -> {n} ({})", lesson.title);
            prev = n;
            last = n;
        }
        assert_eq!(last, 16);
        assert_eq!(next_after(by_id(last_id(Subject::TiengAnh)).unwrap()), None);
    }

    #[test]
    fn next_walks_one_subject() {
        let first = by_id(1).unwrap();
        let second = next_after(first).unwrap();
        assert_eq!(second.id, 2);
        assert_eq!(next_after(by_id(last_id(Subject::Toan)).unwrap()), None);
        assert_eq!(next_after(by_id(55).unwrap()).unwrap().id, 56);
    }

    #[test]
    fn units_group_math() {
        let units: Vec<_> = for_subject(Subject::Toan)
            .into_iter()
            .map(|lesson| lesson.unit)
            .collect();
        assert!(units.contains(&"Các số từ 0 đến 10"));
        assert!(units.contains(&"Cộng trừ trong phạm vi 10"));
        assert!(units.contains(&"Các số đến 100"));
        assert!(units.contains(&"Giờ và lịch"));
    }

    #[test]
    fn viet_and_english_follow_books() {
        let viet: Vec<_> = for_subject(Subject::TiengViet)
            .into_iter()
            .map(|lesson| lesson.title)
            .collect();
        assert!(viet.iter().any(|t| t.contains("Bài 1. A")));
        assert!(viet.iter().any(|t| t.contains("Đèn giao thông")));
        let anh: Vec<_> = for_subject(Subject::TiengAnh)
            .into_iter()
            .map(|lesson| lesson.unit)
            .collect();
        assert!(anh.iter().any(|u| u.contains("Unit 1")));
        assert!(anh.iter().any(|u| u.contains("Unit 16")));
    }
}
'''

(ROOT / "src/lessons.rs").write_text(header + body + footer, encoding="utf-8")
print("ok")
for L in ordered:
    if L["id"] in (1, 30) or L["id"] >= 123:
        print(L["id"], L["unit"], L["title"], L["ok"], L["choices"])
