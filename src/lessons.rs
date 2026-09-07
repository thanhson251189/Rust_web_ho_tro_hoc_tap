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

    pub fn emoji(self) -> &'static str {
        match self {
            Subject::Toan => "🔢",
            Subject::TiengViet => "📖",
            Subject::TiengAnh => "🌈",
        }
    }

    pub fn hint(self) -> &'static str {
        match self {
            Subject::Toan => "Đếm, cộng, trừ",
            Subject::TiengViet => "Chữ cái và từ",
            Subject::TiengAnh => "Hello, colors, animals",
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

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Lesson {
    pub id: u32,
    pub subject: Subject,
    pub title: &'static str,
    pub prompt: &'static str,
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

const LESSONS: [Lesson; 15] = [
    Lesson {
        id: 1,
        subject: Subject::Toan,
        title: "Đếm đến 5",
        prompt: "Có bao nhiêu ngôi sao? ★ ★ ★",
        choices: ["3", "2", "4", "5"],
        correct: 0,
    },
    Lesson {
        id: 2,
        subject: Subject::Toan,
        title: "Số còn thiếu",
        prompt: "1, 2, 3, __, 5. Số nào điền vào chỗ trống?",
        choices: ["2", "6", "4", "0"],
        correct: 2,
    },
    Lesson {
        id: 3,
        subject: Subject::Toan,
        title: "Cộng trong phạm vi 10",
        prompt: "3 + 2 = ?",
        choices: ["4", "6", "32", "5"],
        correct: 3,
    },
    Lesson {
        id: 4,
        subject: Subject::Toan,
        title: "Trừ trong phạm vi 10",
        prompt: "7 − 1 = ?",
        choices: ["6", "5", "8", "71"],
        correct: 0,
    },
    Lesson {
        id: 5,
        subject: Subject::Toan,
        title: "So sánh số",
        prompt: "Số nào lớn hơn?",
        choices: ["4", "2", "9", "1"],
        correct: 2,
    },
    Lesson {
        id: 6,
        subject: Subject::TiengViet,
        title: "Chữ cái A",
        prompt: "Đâu là chữ A?",
        choices: ["A", "O", "U", "I"],
        correct: 0,
    },
    Lesson {
        id: 7,
        subject: Subject::TiengViet,
        title: "Chữ bắt đầu bằng B",
        prompt: "Từ nào bắt đầu bằng chữ B?",
        choices: ["mèo", "cá", "bố", "nhà"],
        correct: 2,
    },
    Lesson {
        id: 8,
        subject: Subject::TiengViet,
        title: "Vần a",
        prompt: "Từ nào có vần a?",
        choices: ["bé", "bố", "bì", "ba"],
        correct: 3,
    },
    Lesson {
        id: 9,
        subject: Subject::TiengViet,
        title: "Đọc từ",
        prompt: "Con vật kêu meo meo là gì?",
        choices: ["chó", "mèo", "gà", "heo"],
        correct: 1,
    },
    Lesson {
        id: 10,
        subject: Subject::TiengViet,
        title: "Câu ngắn",
        prompt: "Câu nào đủ nghĩa?",
        choices: ["là", "cơm", "ăn", "Bé ăn cơm."],
        correct: 3,
    },
    Lesson {
        id: 11,
        subject: Subject::TiengAnh,
        title: "Hello",
        prompt: "Khi gặp bạn, mình nói gì?",
        choices: ["Bye", "Sorry", "Stop", "Hello"],
        correct: 3,
    },
    Lesson {
        id: 12,
        subject: Subject::TiengAnh,
        title: "Colors",
        prompt: "Apple is… (quả táo màu gì?)",
        choices: ["blue", "black", "red", "purple"],
        correct: 2,
    },
    Lesson {
        id: 13,
        subject: Subject::TiengAnh,
        title: "Animals",
        prompt: "Con mèo tiếng Anh là gì?",
        choices: ["cat", "dog", "bird", "fish"],
        correct: 0,
    },
    Lesson {
        id: 14,
        subject: Subject::TiengAnh,
        title: "Numbers",
        prompt: "Số 2 tiếng Anh là gì?",
        choices: ["one", "three", "ten", "two"],
        correct: 3,
    },
    Lesson {
        id: 15,
        subject: Subject::TiengAnh,
        title: "Alphabet",
        prompt: "Chữ cái đầu tiên trong bảng chữ cái tiếng Anh?",
        choices: ["B", "A", "C", "Z"],
        correct: 1,
    },
];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn each_subject_has_five_lessons() {
        for subject in Subject::all() {
            assert_eq!(for_subject(subject).len(), 5);
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
        // Kids must not learn "always tap choice index 1".
        for subject in Subject::all() {
            let mut seen = [false; 4];
            for lesson in for_subject(subject) {
                seen[lesson.correct] = true;
            }
            let distinct = seen.iter().filter(|&&hit| hit).count();
            assert!(
                distinct >= 3,
                "{:?} only uses {} distinct correct slots",
                subject,
                distinct
            );
        }
        let ones = all_lessons()
            .iter()
            .filter(|lesson| lesson.correct == 1)
            .count();
        assert!(
            ones <= 5,
            "too many lessons still use correct index 1 ({ones}/15)"
        );
    }

    #[test]
    fn next_walks_one_subject() {
        let first = by_id(1).unwrap();
        let second = next_after(first).unwrap();
        assert_eq!(second.id, 2);
        assert_eq!(next_after(by_id(5).unwrap()), None);
    }
}
