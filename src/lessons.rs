#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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
            Subject::Toan => "Đếm, so sánh, cộng trừ",
            Subject::TiengViet => "Chữ, vần, từ, câu",
            Subject::TiengAnh => "Hello, số, màu, con vật",
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

const LESSONS: [Lesson; 36] = [
    L!(
        1,
        Subject::Toan,
        "Đếm",
        "Đếm sao",
        "Có bao nhiêu ngôi sao?",
        Picture::Stars(3),
        ["3", "2", "4", "1"],
        0
    ),
    L!(
        2,
        Subject::Toan,
        "Đếm",
        "Đếm táo",
        "Có bao nhiêu quả táo?",
        Picture::Apples(5),
        ["4", "6", "5", "3"],
        2
    ),
    L!(
        3,
        Subject::Toan,
        "Đếm",
        "Đếm chấm",
        "Có bao nhiêu chấm tròn?",
        Picture::Dots(1),
        ["0", "2", "3", "1"],
        3
    ),
    L!(
        4,
        Subject::Toan,
        "Đếm",
        "Không có quả nào",
        "Có bao nhiêu quả táo?",
        Picture::Apples(0),
        ["1", "0", "2", "3"],
        1
    ),
    L!(
        5,
        Subject::Toan,
        "Đếm",
        "Số còn thiếu",
        "1, 2, 3, __, 5. Số nào điền vào chỗ trống?",
        Picture::None,
        ["4", "6", "2", "0"],
        0
    ),
    L!(
        6,
        Subject::Toan,
        "So sánh",
        "Lớn hơn",
        "Số nào lớn hơn?",
        Picture::None,
        ["2", "9", "4", "1"],
        1
    ),
    L!(
        7,
        Subject::Toan,
        "So sánh",
        "Bé hơn",
        "Số nào bé hơn?",
        Picture::None,
        ["8", "6", "3", "7"],
        2
    ),
    L!(
        8,
        Subject::Toan,
        "Phép cộng",
        "Cộng 3 + 2",
        "3 + 2 = ?",
        Picture::Blocks(5),
        ["4", "6", "32", "5"],
        3
    ),
    L!(
        9,
        Subject::Toan,
        "Phép cộng",
        "Cộng 4 + 4",
        "4 + 4 = ?",
        Picture::None,
        ["8", "44", "7", "6"],
        0
    ),
    L!(
        10,
        Subject::Toan,
        "Phép cộng",
        "Cộng 6 + 1",
        "6 + 1 = ?",
        Picture::None,
        ["5", "61", "7", "8"],
        2
    ),
    L!(
        11,
        Subject::Toan,
        "Phép trừ",
        "Trừ 5 − 1",
        "5 − 1 = ?",
        Picture::None,
        ["6", "4", "51", "3"],
        1
    ),
    L!(
        12,
        Subject::Toan,
        "Phép trừ",
        "Trừ 10 − 2",
        "10 − 2 = ?",
        Picture::None,
        ["12", "2", "9", "8"],
        3
    ),
    L!(
        13,
        Subject::TiengViet,
        "Chữ cái",
        "Chữ A",
        "Đâu là chữ A?",
        Picture::None,
        ["O", "A", "U", "I"],
        1
    ),
    L!(
        14,
        Subject::TiengViet,
        "Chữ cái",
        "Chữ B",
        "Đâu là chữ B?",
        Picture::None,
        ["B", "D", "P", "R"],
        0
    ),
    L!(
        15,
        Subject::TiengViet,
        "Chữ cái",
        "Chữ M",
        "Đâu là chữ M?",
        Picture::None,
        ["N", "W", "M", "H"],
        2
    ),
    L!(
        16,
        Subject::TiengViet,
        "Âm đầu",
        "Âm b",
        "Từ nào bắt đầu bằng chữ B?",
        Picture::None,
        ["mèo", "cá", "nhà", "bố"],
        3
    ),
    L!(
        17,
        Subject::TiengViet,
        "Âm đầu",
        "Âm m",
        "Từ nào bắt đầu bằng chữ M?",
        Picture::None,
        ["mẹ", "bố", "cá", "gà"],
        0
    ),
    L!(
        18,
        Subject::TiengViet,
        "Vần",
        "Vần a",
        "Từ nào có vần a?",
        Picture::None,
        ["bé", "ba", "bố", "bì"],
        1
    ),
    L!(
        19,
        Subject::TiengViet,
        "Vần",
        "Vần o",
        "Từ nào có vần o?",
        Picture::None,
        ["bò", "bé", "bì", "ba"],
        0
    ),
    L!(
        20,
        Subject::TiengViet,
        "Vần",
        "Vần ơ",
        "Từ nào có vần ơ?",
        Picture::None,
        ["ba", "bé", "mơ", "bò"],
        2
    ),
    L!(
        21,
        Subject::TiengViet,
        "Từ",
        "Con mèo",
        "Con vật kêu meo meo là gì?",
        Picture::None,
        ["chó", "gà", "heo", "mèo"],
        3
    ),
    L!(
        22,
        Subject::TiengViet,
        "Từ",
        "Ngôi nhà",
        "Chỗ mình ở gọi là gì?",
        Picture::None,
        ["nhà", "cây", "sông", "núi"],
        0
    ),
    L!(
        23,
        Subject::TiengViet,
        "Câu",
        "Câu đủ nghĩa",
        "Câu nào đủ nghĩa?",
        Picture::None,
        ["là", "cơm", "Bé ăn cơm.", "ăn"],
        2
    ),
    L!(
        24,
        Subject::TiengViet,
        "Câu",
        "Ai đang ngủ?",
        "Câu nào hỏi?",
        Picture::None,
        ["Bé ngủ.", "Ai đang ngủ?", "ngủ", "Bé."],
        1
    ),
    L!(
        25,
        Subject::TiengAnh,
        "Greetings",
        "Hello",
        "Khi gặp bạn, mình nói gì?",
        Picture::None,
        ["Bye", "Sorry", "Hello", "Stop"],
        2
    ),
    L!(
        26,
        Subject::TiengAnh,
        "Greetings",
        "Goodbye",
        "Khi chia tay, mình nói gì?",
        Picture::None,
        ["Hello", "Goodbye", "Please", "Red"],
        1
    ),
    L!(
        27,
        Subject::TiengAnh,
        "Alphabet",
        "Letter A",
        "Chữ cái đầu tiên trong bảng chữ cái tiếng Anh?",
        Picture::None,
        ["B", "C", "Z", "A"],
        3
    ),
    L!(
        28,
        Subject::TiengAnh,
        "Alphabet",
        "Letter B",
        "Chữ nào đứng sau A?",
        Picture::None,
        ["B", "D", "Z", "C"],
        0
    ),
    L!(
        29,
        Subject::TiengAnh,
        "Numbers",
        "One",
        "Số 1 tiếng Anh là gì?",
        Picture::Dots(1),
        ["two", "one", "ten", "zero"],
        1
    ),
    L!(
        30,
        Subject::TiengAnh,
        "Numbers",
        "Two",
        "Số 2 tiếng Anh là gì?",
        Picture::Dots(2),
        ["one", "three", "two", "ten"],
        2
    ),
    L!(
        31,
        Subject::TiengAnh,
        "Numbers",
        "Three",
        "Số 3 tiếng Anh là gì?",
        Picture::Stars(3),
        ["five", "tree", "free", "three"],
        3
    ),
    L!(
        32,
        Subject::TiengAnh,
        "Colors",
        "Red",
        "Quả táo chín thường có màu…",
        Picture::Apples(1),
        ["blue", "red", "black", "green"],
        1
    ),
    L!(
        33,
        Subject::TiengAnh,
        "Colors",
        "Yellow",
        "Mặt trời màu gì? (yellow)",
        Picture::Stars(1),
        ["yellow", "blue", "brown", "pink"],
        0
    ),
    L!(
        34,
        Subject::TiengAnh,
        "Animals",
        "Cat",
        "Con mèo tiếng Anh là gì?",
        Picture::None,
        ["dog", "bird", "cat", "fish"],
        2
    ),
    L!(
        35,
        Subject::TiengAnh,
        "Animals",
        "Dog",
        "Con chó tiếng Anh là gì?",
        Picture::None,
        ["dog", "cat", "pig", "cow"],
        0
    ),
    L!(
        36,
        Subject::TiengAnh,
        "Family",
        "Mom",
        "“Mẹ” tiếng Anh thường nói là…",
        Picture::None,
        ["dad", "baby", "mom", "boy"],
        2
    ),
];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn each_subject_has_twelve_lessons() {
        for subject in Subject::all() {
            assert_eq!(for_subject(subject).len(), 12);
        }
    }

    #[test]
    fn correct_index_is_in_range() {
        for lesson in all_lessons() {
            assert!(lesson.correct < lesson.choices.len());
        }
    }

    #[test]
    fn correct_answers_are_not_stuck_on_one_button() {
        for subject in Subject::all() {
            let mut seen = [false; 4];
            for lesson in for_subject(subject) {
                seen[lesson.correct] = true;
            }
            let distinct = seen.iter().filter(|&&hit| hit).count();
            assert!(
                distinct >= 3,
                "{subject:?} only uses {distinct} distinct correct slots"
            );
        }
    }

    #[test]
    fn next_walks_one_subject() {
        let first = by_id(1).unwrap();
        let second = next_after(first).unwrap();
        assert_eq!(second.id, 2);
        assert_eq!(next_after(by_id(last_id(Subject::Toan)).unwrap()), None);
        assert_eq!(next_after(by_id(13).unwrap()).unwrap().id, 14);
    }

    #[test]
    fn units_group_math() {
        let units: Vec<_> = for_subject(Subject::Toan)
            .into_iter()
            .map(|lesson| lesson.unit)
            .collect();
        assert!(units.contains(&"Đếm"));
        assert!(units.contains(&"Phép cộng"));
        assert!(units.contains(&"Phép trừ"));
    }
}
