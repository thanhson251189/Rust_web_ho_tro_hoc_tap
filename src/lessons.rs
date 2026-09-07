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
            Subject::Toan => "Bám SGK KNTT tập 1–2",
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

// Original tap-to-answer items aligned to SGK Toán 1 KNTT (tập 1–2) lesson titles.
// Do not host or copy textbook pages/art.
#[rustfmt::skip]
const LESSONS: [Lesson; 78] = [
    // Chủ đề 1 — Các số từ 0 đến 10
    L!(1, Subject::Toan, "Các số từ 0 đến 10", "Bài 1. Số 0 đến 5", "Có bao nhiêu ngôi sao?", Picture::Stars(3), ["3", "2", "4", "1"], 0),
    L!(2, Subject::Toan, "Các số từ 0 đến 10", "Bài 1. Không có quả nào", "Có bao nhiêu quả táo?", Picture::Apples(0), ["1", "0", "2", "3"], 1),
    L!(3, Subject::Toan, "Các số từ 0 đến 10", "Bài 2. Số 6 đến 10", "Có bao nhiêu quả táo?", Picture::Apples(8), ["7", "9", "10", "8"], 3),
    L!(4, Subject::Toan, "Các số từ 0 đến 10", "Bài 2. Số 10", "10 là số nào?", Picture::None, ["01", "10", "11", "100"], 1),
    L!(5, Subject::Toan, "Các số từ 0 đến 10", "Bài 3. Nhiều hơn", "Nhóm nào nhiều hơn: 5 chấm hay 3 chấm?", Picture::None, ["3 chấm", "bằng nhau", "5 chấm", "không biết"], 2),
    L!(6, Subject::Toan, "Các số từ 0 đến 10", "Bài 3. Ít hơn", "Số nào ít hơn?", Picture::None, ["9", "6", "8", "7"], 1),
    L!(7, Subject::Toan, "Các số từ 0 đến 10", "Bài 3. Bằng nhau", "4 chấm và 4 sao thì thế nào?", Picture::None, ["nhiều hơn", "ít hơn", "bằng nhau", "không so được"], 2),
    L!(8, Subject::Toan, "Các số từ 0 đến 10", "Bài 4. So sánh số", "Số nào lớn hơn?", Picture::None, ["2", "9", "4", "1"], 1),
    L!(9, Subject::Toan, "Các số từ 0 đến 10", "Bài 4. Dấu >", "Chọn dấu đúng: 7 □ 5", Picture::None, [">", "<", "=", "+"], 0),
    L!(10, Subject::Toan, "Các số từ 0 đến 10", "Bài 5. Mấy và mấy", "5 gồm 2 và mấy?", Picture::None, ["2", "4", "3", "5"], 2),
    L!(11, Subject::Toan, "Các số từ 0 đến 10", "Bài 5. Tách 6", "6 gồm 4 và mấy?", Picture::None, ["1", "2", "3", "4"], 1),
    L!(12, Subject::Toan, "Các số từ 0 đến 10", "Bài 6. Luyện tập số", "1, 2, 3, __, 5. Số còn thiếu?", Picture::None, ["4", "6", "2", "0"], 0),
    // Chủ đề 2 — Hình phẳng
    L!(13, Subject::Toan, "Hình phẳng", "Bài 7. Hình tròn", "Hình nào lăn được, không có góc?", Picture::None, ["vuông", "tròn", "tam giác", "chữ nhật"], 1),
    L!(14, Subject::Toan, "Hình phẳng", "Bài 7. Hình vuông", "Hình nào có 4 cạnh bằng nhau?", Picture::None, ["tròn", "tam giác", "vuông", "đường thẳng"], 2),
    L!(15, Subject::Toan, "Hình phẳng", "Bài 7. Hình tam giác", "Tam giác có bao nhiêu cạnh?", Picture::None, ["2", "3", "4", "5"], 1),
    L!(16, Subject::Toan, "Hình phẳng", "Bài 8. Xếp hình", "Ghép 2 tam giác vuông lớn có thể được hình nào?", Picture::None, ["tròn", "vuông", "chấm", "số 8"], 1),
    L!(17, Subject::Toan, "Hình phẳng", "Bài 9. Luyện tập hình", "Hình chữ nhật khác hình vuông ở chỗ nào?", Picture::None, ["có 3 cạnh", "không có góc", "không phải mọi cạnh đều bằng nhau", "là hình tròn"], 2),
    // Chủ đề 3 — Cộng trừ phạm vi 10
    L!(18, Subject::Toan, "Cộng trừ trong phạm vi 10", "Bài 10. Cộng 3 + 2", "3 + 2 = ?", Picture::Blocks(5), ["4", "6", "32", "5"], 3),
    L!(19, Subject::Toan, "Cộng trừ trong phạm vi 10", "Bài 10. Cộng 5 + 2", "5 + 2 = ?", Picture::None, ["6", "7", "52", "3"], 1),
    L!(20, Subject::Toan, "Cộng trừ trong phạm vi 10", "Bài 10. Cộng 4 + 4", "4 + 4 = ?", Picture::None, ["8", "44", "7", "6"], 0),
    L!(21, Subject::Toan, "Cộng trừ trong phạm vi 10", "Bài 11. Trừ 5 − 1", "5 − 1 = ?", Picture::None, ["6", "4", "51", "3"], 1),
    L!(22, Subject::Toan, "Cộng trừ trong phạm vi 10", "Bài 11. Trừ 7 − 3", "7 − 3 = ?", Picture::None, ["4", "10", "3", "5"], 0),
    L!(23, Subject::Toan, "Cộng trừ trong phạm vi 10", "Bài 11. Trừ 10 − 2", "10 − 2 = ?", Picture::None, ["12", "2", "9", "8"], 3),
    L!(24, Subject::Toan, "Cộng trừ trong phạm vi 10", "Bài 12. Bảng cộng", "6 + 1 = ?", Picture::None, ["5", "61", "7", "8"], 2),
    L!(25, Subject::Toan, "Cộng trừ trong phạm vi 10", "Bài 12. Bảng trừ", "9 − 9 = ?", Picture::None, ["9", "1", "0", "18"], 2),
    L!(26, Subject::Toan, "Cộng trừ trong phạm vi 10", "Bài 13. Luyện tập cộng trừ", "8 + 2 = ?", Picture::None, ["9", "10", "6", "82"], 1),
    // Chủ đề 4 — Hình khối
    L!(27, Subject::Toan, "Hình khối", "Bài 14. Khối lập phương", "Viên xúc xắc giống hình nào nhất?", Picture::None, ["khối lập phương", "hình tròn", "tam giác", "đường thẳng"], 0),
    L!(28, Subject::Toan, "Hình khối", "Bài 14. Khối hộp chữ nhật", "Hộp sữa thường giống hình nào?", Picture::None, ["khối cầu", "khối hộp chữ nhật", "tam giác", "số 0"], 1),
    L!(29, Subject::Toan, "Hình khối", "Bài 15. Trên — dưới", "Mái nhà ở đâu so với nền nhà?", Picture::None, ["dưới", "trên", "trong", "sau"], 1),
    L!(30, Subject::Toan, "Hình khối", "Bài 15. Trái — phải", "Khi nhìn ra, tay cầm bút thường là tay nào (với đa số bạn)?", Picture::None, ["tay trái", "tay phải", "hai chân", "không có tay"], 1),
    L!(31, Subject::Toan, "Hình khối", "Bài 16. Luyện tập hình khối", "Quả bóng giống hình nào?", Picture::None, ["vuông", "hộp", "khối cầu", "tam giác"], 2),
    // Chủ đề 5 — Ôn HK1
    L!(32, Subject::Toan, "Ôn tập học kì 1", "Bài 17. Ôn số", "Số liền sau 8 là?", Picture::None, ["7", "9", "10", "18"], 1),
    L!(33, Subject::Toan, "Ôn tập học kì 1", "Bài 18. Ôn phép tính", "9 − 4 = ?", Picture::None, ["5", "13", "4", "6"], 0),
    L!(34, Subject::Toan, "Ôn tập học kì 1", "Bài 19. Ôn hình", "Hình nào có 4 góc vuông?", Picture::None, ["tròn", "tam giác", "vuông", "đường cong"], 2),
    L!(35, Subject::Toan, "Ôn tập học kì 1", "Bài 20. Ôn chung", "Chọn phép tính đúng", Picture::None, ["2 + 2 = 5", "3 + 4 = 7", "10 − 1 = 8", "0 + 1 = 0"], 1),
    // Tập 2 — Chủ đề 6 số đến 100
    L!(36, Subject::Toan, "Các số đến 100", "Bài 21. Số có hai chữ số", "Số 23 gồm bao nhiêu chục và bao nhiêu đơn vị?", Picture::None, ["2 chục 3 đơn vị", "3 chục 2 đơn vị", "23 chục", "5 đơn vị"], 0),
    L!(37, Subject::Toan, "Các số đến 100", "Bài 21. Đọc số", "Số 40 đọc là?", Picture::None, ["bốn", "mười bốn", "bốn mươi", "bốn trăm"], 2),
    L!(38, Subject::Toan, "Các số đến 100", "Bài 22. So sánh số hai chữ số", "Số nào lớn hơn?", Picture::None, ["19", "91", "18", "29"], 1),
    L!(39, Subject::Toan, "Các số đến 100", "Bài 23. Bảng số 1–100", "Số đứng ngay sau 29 là?", Picture::None, ["28", "39", "30", "20"], 2),
    L!(40, Subject::Toan, "Các số đến 100", "Bài 24. Luyện tập số 100", "100 gồm mấy chục?", Picture::None, ["1", "10", "100", "0"], 1),
    // Chủ đề 7 đo độ dài
    L!(41, Subject::Toan, "Độ dài", "Bài 25. Dài hơn", "Cái thước thường dài hơn cái gì?", Picture::None, ["cây bút chì", "con đường", "sông", "nhà cao tầng"], 0),
    L!(42, Subject::Toan, "Độ dài", "Bài 26. Xăng-ti-mét", "Đơn vị đo độ dài bé học ở lớp 1 thường là?", Picture::None, ["kg", "giờ", "cm", "lít"], 2),
    L!(43, Subject::Toan, "Độ dài", "Bài 27. Ước lượng", "Bút chì khoảng bao nhiêu xăng-ti-mét?", Picture::None, ["1 cm", "15 cm", "100 cm", "1 km"], 1),
    // Chủ đề 8 cộng trừ không nhớ phạm vi 100
    L!(44, Subject::Toan, "Cộng trừ không nhớ phạm vi 100", "Bài 29. Cộng 32 + 5", "32 + 5 = ?", Picture::None, ["37", "325", "27", "42"], 0),
    L!(45, Subject::Toan, "Cộng trừ không nhớ phạm vi 100", "Bài 30. Cộng 20 + 10", "20 + 10 = ?", Picture::None, ["21", "2010", "30", "12"], 2),
    L!(46, Subject::Toan, "Cộng trừ không nhớ phạm vi 100", "Bài 31. Trừ 45 − 3", "45 − 3 = ?", Picture::None, ["48", "42", "15", "43"], 1),
    L!(47, Subject::Toan, "Cộng trừ không nhớ phạm vi 100", "Bài 32. Trừ 40 − 10", "40 − 10 = ?", Picture::None, ["30", "50", "4010", "4"], 0),
    L!(48, Subject::Toan, "Cộng trừ không nhớ phạm vi 100", "Bài 33. Luyện tập", "25 + 4 = ?", Picture::None, ["21", "254", "29", "30"], 2),
    // Chủ đề 9 giờ và lịch
    L!(49, Subject::Toan, "Giờ và lịch", "Bài 34. Xem giờ đúng", "Kim ngắn chỉ số 3, kim dài chỉ số 12. Mấy giờ?", Picture::None, ["12 giờ", "3 giờ", "6 giờ", "9 giờ"], 1),
    L!(50, Subject::Toan, "Giờ và lịch", "Bài 35. Ngày trong tuần", "Ngày đứng sau thứ Hai là?", Picture::None, ["Chủ nhật", "thứ Ba", "thứ Bảy", "thứ Sáu"], 1),
    L!(51, Subject::Toan, "Giờ và lịch", "Bài 36. Xem lịch", "Một tuần có bao nhiêu ngày?", Picture::None, ["5", "6", "7", "10"], 2),
    // Chủ đề 10 ôn cuối năm
    L!(52, Subject::Toan, "Ôn tập cuối năm", "Bài 38. Ôn phạm vi 10", "6 + 3 = ?", Picture::None, ["8", "9", "3", "63"], 1),
    L!(53, Subject::Toan, "Ôn tập cuối năm", "Bài 39. Ôn phạm vi 100", "50 + 20 = ?", Picture::None, ["30", "70", "5020", "52"], 1),
    L!(54, Subject::Toan, "Ôn tập cuối năm", "Bài 40. Ôn đo lường", "Đồng hồ dùng để làm gì?", Picture::None, ["đo độ dài", "xem giờ", "đếm táo", "vẽ hình"], 1),
    // Tiếng Việt — starter (ids 55–66)
    L!(55, Subject::TiengViet, "Chữ cái", "Chữ A", "Đâu là chữ A?", Picture::None, ["O", "A", "U", "I"], 1),
    L!(56, Subject::TiengViet, "Chữ cái", "Chữ B", "Đâu là chữ B?", Picture::None, ["B", "D", "P", "R"], 0),
    L!(57, Subject::TiengViet, "Chữ cái", "Chữ M", "Đâu là chữ M?", Picture::None, ["N", "W", "M", "H"], 2),
    L!(58, Subject::TiengViet, "Âm đầu", "Âm b", "Từ nào bắt đầu bằng chữ B?", Picture::None, ["mèo", "cá", "nhà", "bố"], 3),
    L!(59, Subject::TiengViet, "Âm đầu", "Âm m", "Từ nào bắt đầu bằng chữ M?", Picture::None, ["mẹ", "bố", "cá", "gà"], 0),
    L!(60, Subject::TiengViet, "Vần", "Vần a", "Từ nào có vần a?", Picture::None, ["bé", "ba", "bố", "bì"], 1),
    L!(61, Subject::TiengViet, "Vần", "Vần o", "Từ nào có vần o?", Picture::None, ["bò", "bé", "bì", "ba"], 0),
    L!(62, Subject::TiengViet, "Vần", "Vần ơ", "Từ nào có vần ơ?", Picture::None, ["ba", "bé", "mơ", "bò"], 2),
    L!(63, Subject::TiengViet, "Từ", "Con mèo", "Con vật kêu meo meo là gì?", Picture::None, ["chó", "gà", "heo", "mèo"], 3),
    L!(64, Subject::TiengViet, "Từ", "Ngôi nhà", "Chỗ mình ở gọi là gì?", Picture::None, ["nhà", "cây", "sông", "núi"], 0),
    L!(65, Subject::TiengViet, "Câu", "Câu đủ nghĩa", "Câu nào đủ nghĩa?", Picture::None, ["là", "cơm", "Bé ăn cơm.", "ăn"], 2),
    L!(66, Subject::TiengViet, "Câu", "Câu hỏi", "Câu nào hỏi?", Picture::None, ["Bé ngủ.", "Ai đang ngủ?", "ngủ", "Bé."], 1),
    // Tiếng Anh — starter (ids 67–78)
    L!(67, Subject::TiengAnh, "Greetings", "Hello", "Khi gặp bạn, mình nói gì?", Picture::None, ["Bye", "Sorry", "Hello", "Stop"], 2),
    L!(68, Subject::TiengAnh, "Greetings", "Goodbye", "Khi chia tay, mình nói gì?", Picture::None, ["Hello", "Goodbye", "Please", "Red"], 1),
    L!(69, Subject::TiengAnh, "Alphabet", "Letter A", "Chữ cái đầu tiên trong bảng chữ cái tiếng Anh?", Picture::None, ["B", "C", "Z", "A"], 3),
    L!(70, Subject::TiengAnh, "Alphabet", "Letter B", "Chữ nào đứng sau A?", Picture::None, ["B", "D", "Z", "C"], 0),
    L!(71, Subject::TiengAnh, "Numbers", "One", "Số 1 tiếng Anh là gì?", Picture::Dots(1), ["two", "one", "ten", "zero"], 1),
    L!(72, Subject::TiengAnh, "Numbers", "Two", "Số 2 tiếng Anh là gì?", Picture::Dots(2), ["one", "three", "two", "ten"], 2),
    L!(73, Subject::TiengAnh, "Numbers", "Three", "Số 3 tiếng Anh là gì?", Picture::Stars(3), ["five", "tree", "free", "three"], 3),
    L!(74, Subject::TiengAnh, "Colors", "Red", "Quả táo chín thường có màu…", Picture::Apples(1), ["blue", "red", "black", "green"], 1),
    L!(75, Subject::TiengAnh, "Colors", "Yellow", "Mặt trời màu gì? (yellow)", Picture::Stars(1), ["yellow", "blue", "brown", "pink"], 0),
    L!(76, Subject::TiengAnh, "Animals", "Cat", "Con mèo tiếng Anh là gì?", Picture::None, ["dog", "bird", "cat", "fish"], 2),
    L!(77, Subject::TiengAnh, "Animals", "Dog", "Con chó tiếng Anh là gì?", Picture::None, ["dog", "cat", "pig", "cow"], 0),
    L!(78, Subject::TiengAnh, "Family", "Mom", "“Mẹ” tiếng Anh thường nói là…", Picture::None, ["dad", "baby", "mom", "boy"], 2),
];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn math_follows_sgk_length() {
        assert_eq!(for_subject(Subject::Toan).len(), 54);
        assert_eq!(for_subject(Subject::TiengViet).len(), 12);
        assert_eq!(for_subject(Subject::TiengAnh).len(), 12);
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
}
