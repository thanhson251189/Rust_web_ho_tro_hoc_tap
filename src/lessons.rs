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
pub enum FlatShape {
    Circle,
    Square,
    Triangle,
    Rectangle,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SolidShape {
    Cube,
    Cuboid,
    Sphere,
}

/// Scenes shown next to each question. All art is drawn in code (original
/// SVG, SGK-pedagogy inspired) — no textbook scans are hosted.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Picture {
    None,
    Stars(u8),
    Apples(u8),
    Dots(u8),
    Blocks(u8),
    /// Two groups side by side to compare: (a, b).
    Compare(u8, u8),
    /// Two groups to join with a plus sign: (a, b).
    Join(u8, u8),
    /// Bags of ten tomatoes plus loose ones: (chuc, don-vi).
    Tomatoes(u8, u8),
    /// The 1-100 chart with `hi` highlighted and `hi + 1` shown as "?".
    Chart100(u32),
    /// Clock face at (hour, minute).
    Clock(u8, u8),
    /// The seven days of the week as chips (Thu Hai highlighted).
    Week,
    /// One shape character.
    FlatShape(FlatShape),
    /// All four flat shapes in a row.
    FlatShapes,
    SolidShape(SolidShape),
    /// Ruler with a pencil above it.
    Ruler,
    /// A single big emoji standing for the object in the question.
    Emoji(&'static str),
    /// SGK-style letter card: uppercase + lowercase forms of one letter.
    LetterCard(&'static str, &'static str),
    /// SGK-style rhyme card: the rhyme plus a sample word that contains it.
    RhymeCard(&'static str, &'static str),
    /// English phonics card: letter, word, and the emoji shown with it.
    WordCard(&'static str, &'static str, &'static str),
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
#[allow(clippy::large_const_arrays)]
const LESSONS: [Lesson; 201] = [
    // Chủ đề 1 — Các số từ 0 đến 10
    L!(1, Subject::Toan, "Các số từ 0 đến 10", "Bài 1. Số 0 đến 5", "Có bao nhiêu ngôi sao?", Picture::Stars(3), ["3", "2", "4", "1"], 0),
    L!(2, Subject::Toan, "Các số từ 0 đến 10", "Bài 1. Không có quả nào", "Có bao nhiêu quả táo?", Picture::Apples(0), ["1", "0", "2", "3"], 1),
    L!(3, Subject::Toan, "Các số từ 0 đến 10", "Bài 2. Số 6 đến 10", "Có bao nhiêu quả táo?", Picture::Apples(8), ["7", "9", "8", "10"], 2),
    L!(4, Subject::Toan, "Các số từ 0 đến 10", "Bài 2. Số 10", "10 là số nào?", Picture::None, ["01", "11", "100", "10"], 3),
    L!(5, Subject::Toan, "Các số từ 0 đến 10", "Bài 3. Nhiều hơn", "Nhóm nào nhiều hơn: 5 chấm hay 3 chấm?", Picture::Compare(5, 3), ["3 chấm", "5 chấm", "bằng nhau", "không biết"], 1),
    L!(6, Subject::Toan, "Các số từ 0 đến 10", "Bài 3. Ít hơn", "Số nào ít hơn?", Picture::Compare(6, 7), ["6", "9", "8", "7"], 0),
    L!(7, Subject::Toan, "Các số từ 0 đến 10", "Bài 3. Bằng nhau", "4 chấm và 4 sao thì thế nào?", Picture::Compare(4, 4), ["nhiều hơn", "ít hơn", "bằng nhau", "không so được"], 2),
    L!(8, Subject::Toan, "Các số từ 0 đến 10", "Bài 4. So sánh số", "Số nào lớn hơn?", Picture::None, ["2", "4", "1", "9"], 3),
    L!(9, Subject::Toan, "Các số từ 0 đến 10", "Bài 4. Dấu >", "Chọn dấu đúng: 7 □ 5", Picture::Compare(7, 5), [">", "<", "=", "+"], 0),
    L!(10, Subject::Toan, "Các số từ 0 đến 10", "Bài 5. Mấy và mấy", "5 gồm 2 và mấy?", Picture::None, ["2", "4", "3", "5"], 2),
    L!(11, Subject::Toan, "Các số từ 0 đến 10", "Bài 5. Tách 6", "6 gồm 4 và mấy?", Picture::None, ["1", "2", "3", "4"], 1),
    L!(12, Subject::Toan, "Các số từ 0 đến 10", "Bài 6. Luyện tập số", "1, 2, 3, __, 5. Số còn thiếu?", Picture::None, ["6", "2", "0", "4"], 3),
    // Chủ đề 2 — Hình phẳng
    L!(13, Subject::Toan, "Hình phẳng", "Bài 7. Hình tròn", "Hình nào lăn được, không có góc?", Picture::FlatShapes, ["tròn", "vuông", "tam giác", "chữ nhật"], 0),
    L!(14, Subject::Toan, "Hình phẳng", "Bài 7. Hình vuông", "Hình nào có 4 cạnh bằng nhau?", Picture::FlatShapes, ["tròn", "vuông", "tam giác", "đường thẳng"], 1),
    L!(15, Subject::Toan, "Hình phẳng", "Bài 7. Hình tam giác", "Tam giác có bao nhiêu cạnh?", Picture::FlatShape(FlatShape::Triangle), ["2", "4", "5", "3"], 3),
    L!(16, Subject::Toan, "Hình phẳng", "Bài 8. Xếp hình", "Ghép 2 tam giác vuông lớn có thể được hình nào?", Picture::None, ["tròn", "chấm", "vuông", "số 8"], 2),
    L!(17, Subject::Toan, "Hình phẳng", "Bài 9. Luyện tập hình", "Hình chữ nhật khác hình vuông ở chỗ nào?", Picture::FlatShapes, ["không phải mọi cạnh đều bằng nhau", "có 3 cạnh", "không có góc", "là hình tròn"], 0),
    // Chủ đề 3 — Cộng trừ phạm vi 10
    L!(18, Subject::Toan, "Cộng trừ trong phạm vi 10", "Bài 10. Cộng 3 + 2", "3 + 2 = ?", Picture::Blocks(5), ["4", "5", "6", "32"], 1),
    L!(19, Subject::Toan, "Cộng trừ trong phạm vi 10", "Bài 10. Cộng 5 + 2", "5 + 2 = ?", Picture::Join(5, 2), ["6", "52", "7", "3"], 2),
    L!(20, Subject::Toan, "Cộng trừ trong phạm vi 10", "Bài 10. Cộng 4 + 4", "4 + 4 = ?", Picture::Join(4, 4), ["8", "44", "7", "6"], 0),
    L!(21, Subject::Toan, "Cộng trừ trong phạm vi 10", "Bài 11. Trừ 5 − 1", "5 − 1 = ?", Picture::None, ["6", "51", "3", "4"], 3),
    L!(22, Subject::Toan, "Cộng trừ trong phạm vi 10", "Bài 11. Trừ 7 − 3", "7 − 3 = ?", Picture::None, ["10", "4", "3", "5"], 1),
    L!(23, Subject::Toan, "Cộng trừ trong phạm vi 10", "Bài 11. Trừ 10 − 2", "10 − 2 = ?", Picture::None, ["12", "2", "8", "9"], 2),
    L!(24, Subject::Toan, "Cộng trừ trong phạm vi 10", "Bài 12. Bảng cộng", "6 + 1 = ?", Picture::None, ["5", "61", "8", "7"], 3),
    L!(25, Subject::Toan, "Cộng trừ trong phạm vi 10", "Bài 12. Bảng trừ", "9 − 9 = ?", Picture::None, ["9", "0", "1", "18"], 1),
    L!(26, Subject::Toan, "Cộng trừ trong phạm vi 10", "Bài 13. Luyện tập cộng trừ", "8 + 2 = ?", Picture::None, ["10", "9", "6", "82"], 0),
    // Chủ đề 4 — Hình khối
    L!(27, Subject::Toan, "Hình khối", "Bài 14. Khối lập phương", "Viên xúc xắc giống hình nào nhất?", Picture::SolidShape(SolidShape::Cube), ["hình tròn", "tam giác", "khối lập phương", "đường thẳng"], 2),
    L!(28, Subject::Toan, "Hình khối", "Bài 14. Khối hộp chữ nhật", "Hộp sữa thường giống hình nào?", Picture::SolidShape(SolidShape::Cuboid), ["khối cầu", "tam giác", "số 0", "khối hộp chữ nhật"], 3),
    L!(29, Subject::Toan, "Hình khối", "Bài 15. Trên — dưới", "Mái nhà ở đâu so với nền nhà?", Picture::None, ["trên", "dưới", "trong", "sau"], 0),
    L!(30, Subject::Toan, "Hình khối", "Bài 15. Trái — phải", "Trên thước, số 2 nằm bên trái số 3. Số 4 nằm bên nào so với số 3?", Picture::Ruler, ["trên", "dưới", "phải", "giữa"], 2),
    L!(31, Subject::Toan, "Hình khối", "Bài 16. Luyện tập hình khối", "Quả bóng giống hình nào?", Picture::SolidShape(SolidShape::Sphere), ["vuông", "khối cầu", "hộp", "tam giác"], 1),
    // Chủ đề 5 — Ôn HK1
    L!(32, Subject::Toan, "Ôn tập học kì 1", "Bài 17. Ôn số", "Số liền sau 8 là?", Picture::None, ["7", "10", "18", "9"], 3),
    L!(33, Subject::Toan, "Ôn tập học kì 1", "Bài 18. Ôn phép tính", "9 − 4 = ?", Picture::None, ["5", "13", "4", "6"], 0),
    L!(34, Subject::Toan, "Ôn tập học kì 1", "Bài 19. Ôn hình", "Hình nào có 4 góc vuông?", Picture::FlatShapes, ["tròn", "vuông", "tam giác", "đường cong"], 1),
    L!(35, Subject::Toan, "Ôn tập học kì 1", "Bài 20. Ôn chung", "Chọn phép tính đúng", Picture::None, ["2 + 2 = 5", "10 − 1 = 8", "0 + 1 = 0", "3 + 4 = 7"], 3),
    // Tập 2 — Chủ đề 6 số đến 100
    L!(36, Subject::Toan, "Các số đến 100", "Bài 21. Số có hai chữ số", "Số 23 gồm bao nhiêu chục và bao nhiêu đơn vị?", Picture::Tomatoes(2, 3), ["3 chục 2 đơn vị", "23 chục", "2 chục 3 đơn vị", "5 đơn vị"], 2),
    L!(37, Subject::Toan, "Các số đến 100", "Bài 21. Đọc số", "Số 40 đọc là?", Picture::None, ["bốn mươi", "bốn", "mười bốn", "bốn trăm"], 0),
    L!(38, Subject::Toan, "Các số đến 100", "Bài 22. So sánh số hai chữ số", "Số nào lớn hơn?", Picture::None, ["19", "91", "18", "29"], 1),
    L!(39, Subject::Toan, "Các số đến 100", "Bài 23. Bảng số 1–100", "Số đứng ngay sau 29 là?", Picture::Chart100(29), ["28", "39", "30", "20"], 2),
    L!(40, Subject::Toan, "Các số đến 100", "Bài 24. Luyện tập số 100", "100 gồm mấy chục?", Picture::Tomatoes(10, 0), ["10", "1", "100", "0"], 0),
    // Chủ đề 7 đo độ dài
    L!(41, Subject::Toan, "Độ dài", "Bài 25. Dài hơn", "Cái thước thường dài hơn cái gì?", Picture::Ruler, ["con đường", "sông", "nhà cao tầng", "cây bút chì"], 3),
    L!(42, Subject::Toan, "Độ dài", "Bài 26. Xăng-ti-mét", "Đơn vị đo độ dài bé học ở lớp 1 thường là?", Picture::Ruler, ["kg", "cm", "giờ", "lít"], 1),
    L!(43, Subject::Toan, "Độ dài", "Bài 27. Ước lượng", "Bút chì khoảng bao nhiêu xăng-ti-mét?", Picture::Ruler, ["1 cm", "100 cm", "15 cm", "1 km"], 2),
    // Chủ đề 8 cộng trừ không nhớ phạm vi 100
    L!(44, Subject::Toan, "Cộng trừ không nhớ phạm vi 100", "Bài 29. Cộng 32 + 5", "32 + 5 = ?", Picture::None, ["325", "27", "42", "37"], 3),
    L!(45, Subject::Toan, "Cộng trừ không nhớ phạm vi 100", "Bài 30. Cộng 20 + 10", "20 + 10 = ?", Picture::None, ["21", "30", "2010", "12"], 1),
    L!(46, Subject::Toan, "Cộng trừ không nhớ phạm vi 100", "Bài 31. Trừ 45 − 3", "45 − 3 = ?", Picture::None, ["42", "48", "15", "43"], 0),
    L!(47, Subject::Toan, "Cộng trừ không nhớ phạm vi 100", "Bài 32. Trừ 40 − 10", "40 − 10 = ?", Picture::None, ["50", "4010", "30", "4"], 2),
    L!(48, Subject::Toan, "Cộng trừ không nhớ phạm vi 100", "Bài 33. Luyện tập", "25 + 4 = ?", Picture::None, ["21", "254", "30", "29"], 3),
    // Chủ đề 9 giờ và lịch
    L!(49, Subject::Toan, "Giờ và lịch", "Bài 34. Xem giờ đúng", "Kim ngắn chỉ số 3, kim dài chỉ số 12. Mấy giờ?", Picture::Clock(3, 0), ["3 giờ", "12 giờ", "6 giờ", "9 giờ"], 0),
    L!(50, Subject::Toan, "Giờ và lịch", "Bài 35. Ngày trong tuần", "Ngày đứng sau thứ Hai là?", Picture::Week, ["Chủ nhật", "thứ Bảy", "thứ Ba", "thứ Sáu"], 2),
    L!(51, Subject::Toan, "Giờ và lịch", "Bài 36. Xem lịch", "Một tuần có bao nhiêu ngày?", Picture::Week, ["5", "7", "6", "10"], 1),
    // Chủ đề 10 ôn cuối năm
    L!(52, Subject::Toan, "Ôn tập cuối năm", "Bài 38. Ôn phạm vi 10", "6 + 3 = ?", Picture::Apples(9), ["8", "3", "63", "9"], 3),
    L!(53, Subject::Toan, "Ôn tập cuối năm", "Bài 39. Ôn phạm vi 100", "50 + 20 = ?", Picture::None, ["70", "30", "5020", "52"], 0),
    L!(54, Subject::Toan, "Ôn tập cuối năm", "Bài 40. Ôn đo lường", "Đồng hồ dùng để làm gì?", Picture::Clock(9, 0), ["đo độ dài", "xem giờ", "đếm táo", "vẽ hình"], 1),
    L!(127, Subject::Toan, "Độ dài", "Bài 28. Luyện tập chung", "Thước nào dài hơn: thước 15 cm hay thước 30 cm?", Picture::None, ["thước 15 cm", "bằng nhau", "không đo được", "thước 30 cm"], 3),
    L!(128, Subject::Toan, "Độ dài", "Bài 28. Luyện tập chung", "1 gang tay của bé khoảng bao nhiêu?", Picture::None, ["1 cm", "1 m", "10 cm", "10 m"], 2),
    L!(129, Subject::Toan, "Giờ và lịch", "Bài 37. Luyện tập chung", "Kim phút chỉ số 12, kim giờ chỉ số 6. Mấy giờ?", Picture::None, ["6 giờ", "12 giờ", "6 phút", "nửa giờ"], 0),
    L!(130, Subject::Toan, "Giờ và lịch", "Bài 37. Luyện tập chung", "Ngày đứng trước thứ Tư là?", Picture::None, ["thứ Năm", "thứ Ba", "thứ Sáu", "Chủ nhật"], 1),
    L!(131, Subject::Toan, "Ôn tập cuối năm", "Bài 41. Ôn tập chung", "10 + 5 = ?", Picture::None, ["51", "14", "15", "16"], 2),
    L!(132, Subject::Toan, "Ôn tập cuối năm", "Bài 41. Ôn tập chung", "Nhóm 7 chấm và 7 sao thì thế nào?", Picture::None, ["bằng nhau", "7 chấm nhiều hơn", "7 sao nhiều hơn", "không so được"], 0),
    L!(133, Subject::Toan, "Ôn tập cuối năm", "Bài 41. Ôn tập chung", "Số nào nhỏ hơn 68?", Picture::None, ["70", "86", "68", "59"], 3),
    L!(134, Subject::Toan, "Ôn tập cuối năm", "Bài 41. Ôn tập chung", "Kim ngắn chỉ 9, kim dài chỉ 12. Mấy giờ?", Picture::None, ["12 giờ", "9 giờ", "9 phút", "3 giờ"], 1),
    // Tiếng Việt KNTT tập 1 — chữ và vần
    L!(55, Subject::TiengViet, "Chữ cái", "Bài 1. A", "Đâu là chữ A?", Picture::LetterCard("A", "a"), ["A", "O", "U", "I"], 0),
    L!(56, Subject::TiengViet, "Chữ cái", "Bài 2. B", "Từ nào có chữ b?", Picture::RhymeCard("b", "bò"), ["mẹ", "bò", "cá", "nhà"], 1),
    L!(57, Subject::TiengViet, "Chữ cái", "Bài 3. C", "Đâu là chữ C?", Picture::LetterCard("C", "c"), ["O", "G", "C", "Q"], 2),
    L!(58, Subject::TiengViet, "Chữ cái", "Bài 4. Ê", "Đâu là chữ Ê?", Picture::LetterCard("Ê", "ê"), ["E", "A", "Ô", "Ê"], 3),
    L!(59, Subject::TiengViet, "Chữ cái", "Bài 6. O", "Từ nào có chữ o?", Picture::RhymeCard("o", "bò"), ["bé", "bò", "mẹ", "bì"], 1),
    L!(60, Subject::TiengViet, "Chữ cái", "Bài 7. Ô", "Đâu là chữ Ô?", Picture::LetterCard("Ô", "ô"), ["Ô", "O", "Ơ", "U"], 0),
    L!(61, Subject::TiengViet, "Chữ cái", "Bài 8. Đ", "Đâu là chữ Đ?", Picture::LetterCard("Đ", "đ"), ["D", "B", "Đ", "P"], 2),
    L!(62, Subject::TiengViet, "Chữ cái", "Bài 9. Ơ", "Từ nào có chữ ơ?", Picture::RhymeCard("ơ", "mơ"), ["ba", "bò", "bé", "mơ"], 3),
    L!(63, Subject::TiengViet, "Chữ cái", "Bài 11. I", "Đâu là chữ I?", Picture::LetterCard("I", "i"), ["I", "L", "T", "J"], 0),
    L!(64, Subject::TiengViet, "Chữ cái", "Bài 12. L", "Từ nào bắt đầu bằng l?", Picture::RhymeCard("l", "lá"), ["cá", "bò", "lá", "mẹ"], 2),
    L!(65, Subject::TiengViet, "Chữ cái", "Bài 13. Ư", "Đâu là chữ Ư?", Picture::LetterCard("Ư", "ư"), ["U", "Ư", "I", "Ô"], 1),
    L!(66, Subject::TiengViet, "Âm ghép", "Bài 14. Ch", "Từ nào có ch?", Picture::RhymeCard("ch", "cha"), ["cá", "lá", "bò", "cha"], 3),
    L!(67, Subject::TiengViet, "Chữ cái", "Bài 16. M", "Từ nào bắt đầu bằng m?", Picture::RhymeCard("m", "mẹ"), ["mẹ", "bố", "cá", "gà"], 0),
    L!(68, Subject::TiengViet, "Chữ cái", "Bài 17. G", "Từ nào bắt đầu bằng g?", Picture::RhymeCard("g", "gà"), ["nhà", "gà", "mèo", "bố"], 1),
    L!(69, Subject::TiengViet, "Âm ghép", "Bài 18. Nh", "Từ nào có nh?", Picture::RhymeCard("nh", "nhà"), ["cá", "bò", "lá", "nhà"], 3),
    L!(70, Subject::TiengViet, "Âm ghép", "Bài 19. Ng", "Từ nào có ng?", Picture::RhymeCard("ng", "ngựa"), ["cá", "mèo", "ngựa", "lá"], 2),
    L!(71, Subject::TiengViet, "Chữ cái", "Bài 21. S", "Đâu là chữ S?", Picture::LetterCard("S", "s"), ["S", "X", "Z", "C"], 0),
    L!(72, Subject::TiengViet, "Chữ cái", "Bài 22. T", "Từ nào bắt đầu bằng t?", Picture::RhymeCard("t", "tàu"), ["mẹ", "tàu", "bò", "nhà"], 1),
    L!(73, Subject::TiengViet, "Âm ghép", "Bài 23. Th", "Từ nào có th?", Picture::RhymeCard("th", "thỏ"), ["cá", "bò", "thỏ", "lá"], 2),
    L!(74, Subject::TiengViet, "Vần", "Bài 24. ưa", "Từ nào có vần ưa?", Picture::RhymeCard("ưa", "lúa"), ["lúa", "ba", "bò", "mẹ"], 0),
    L!(75, Subject::TiengViet, "Âm ghép", "Bài 26. Qu", "Từ nào có qu?", Picture::RhymeCard("qu", "quả"), ["cá", "lá", "bò", "quả"], 3),
    L!(76, Subject::TiengViet, "Chữ cái", "Bài 27. V", "Từ nào bắt đầu bằng v?", Picture::RhymeCard("v", "voi"), ["mèo", "voi", "cá", "nhà"], 1),
    L!(77, Subject::TiengViet, "Chữ cái", "Bài 28. Y", "Đâu là chữ Y?", Picture::LetterCard("Y", "y"), ["V", "U", "Y", "I"], 2),
    L!(78, Subject::TiengViet, "Vần", "Bài 31. an", "Từ nào có vần an?", Picture::RhymeCard("an", "bạn"), ["bò", "mơ", "lá", "bạn"], 3),
    L!(79, Subject::TiengViet, "Vần", "Bài 32. on", "Từ nào có vần on?", Picture::RhymeCard("on", "con"), ["cá", "con", "mẹ", "lá"], 1),
    L!(80, Subject::TiengViet, "Vần", "Bài 34. am", "Từ nào có vần am?", Picture::RhymeCard("am", "năm"), ["năm", "bò", "mơ", "cá"], 0),
    L!(81, Subject::TiengViet, "Vần", "Bài 36. ơm", "Từ nào có vần ơm?", Picture::RhymeCard("ơm", "cơm"), ["ba", "bò", "cơm", "lá"], 2),
    L!(82, Subject::TiengViet, "Vần", "Bài 38. ai", "Từ nào có vần ai?", Picture::RhymeCard("ai", "tai"), ["bò", "mẹ", "cơm", "tai"], 3),
    L!(83, Subject::TiengViet, "Vần", "Bài 39. ơi", "Từ nào có vần ơi?", Picture::RhymeCard("ơi", "tôi"), ["tôi", "ba", "cá", "lá"], 0),
    L!(84, Subject::TiengViet, "Vần", "Bài 42. ao", "Từ nào có vần ao?", Picture::RhymeCard("ao", "sao"), ["mẹ", "bò", "sao", "cơm"], 2),
    L!(85, Subject::TiengViet, "Vần", "Bài 43. âu", "Từ nào có vần âu?", Picture::RhymeCard("âu", "câu"), ["ba", "câu", "bò", "lá"], 1),
    L!(86, Subject::TiengViet, "Ôn tập tập 1", "Chữ thường a", "Đâu là chữ a viết thường?", Picture::LetterCard("a", "a"), ["A", "Ă", "Â", "a"], 3),
    // Tiếng Việt KNTT tập 2 — đọc hiểu ngắn
    L!(87, Subject::TiengViet, "Tôi và các bạn", "Tôi là học sinh lớp 1", "Bé học lớp mấy?", Picture::Emoji("\u{1F392}"), ["lớp 1", "lớp 5", "lớp 9", "mẫu giáo"], 0),
    L!(88, Subject::TiengViet, "Điều em cần biết", "Lời chào", "Khi gặp thầy cô, bé nói gì?", Picture::Emoji("\u{1F44B}"), ["im lặng", "chào ạ", "chạy đi", "quay lưng"], 1),
    L!(89, Subject::TiengViet, "Điều em cần biết", "Rửa tay", "Rửa tay khi nào?", Picture::Emoji("\u{1F9FC}"), ["sau khi ăn no", "khi ngủ", "khi xem TV", "trước khi ăn"], 3),
    L!(90, Subject::TiengViet, "Điều em cần biết", "Đèn giao thông", "Đèn đỏ nghĩa là gì?", Picture::Emoji("\u{1F6A6}"), ["đi", "chạy nhanh", "dừng lại", "bật nhạc"], 2),
    L!(91, Subject::TiengViet, "Mái ấm gia đình", "Ngôi nhà", "Chỗ mình ở gọi là gì?", Picture::Emoji("\u{1F3E0}"), ["nhà", "sông", "núi", "chợ"], 0),
    L!(92, Subject::TiengViet, "Mái trường mến yêu", "Giờ ra chơi", "Giờ ra chơi thường làm gì?", Picture::Emoji("\u{26BD}"), ["ngủ ở nhà", "vui với bạn", "đi chợ", "lái xe"], 1),
    L!(93, Subject::TiengViet, "Tôi và các bạn", "Tình bạn", "Bạn tốt thì nên thế nào?", Picture::Emoji("\u{1F91D}"), ["cãi nhau", "giấu đồ", "giúp đỡ nhau", "im mãi"], 2),
    L!(94, Subject::TiengViet, "Câu", "Câu đủ nghĩa", "Câu nào đủ nghĩa?", Picture::Emoji("\u{270F}\u{FE0F}"), ["Bé ăn cơm.", "là", "cơm", "ăn"], 0),
    L!(135, Subject::TiengViet, "Chữ cái", "Bài 4. E", "Đâu là chữ E?", Picture::LetterCard("E", "e"), ["F", "L", "T", "E"], 3),
    L!(136, Subject::TiengViet, "Chữ cái", "Bài 11. K", "Đâu là chữ K?", Picture::LetterCard("K", "k"), ["X", "K", "Y", "H"], 1),
    L!(137, Subject::TiengViet, "Chữ cái", "Bài 12. H", "Từ nào bắt đầu bằng h?", Picture::RhymeCard("h", "hoa"), ["cá", "bò", "hoa", "mẹ"], 2),
    L!(138, Subject::TiengViet, "Chữ cái", "Bài 16. N", "Từ nào bắt đầu bằng n?", Picture::RhymeCard("n", "nón"), ["bút", "cá", "gà", "nón"], 3),
    L!(139, Subject::TiengViet, "Chữ cái", "Bài 18. Gh", "Từ nào có gh?", Picture::RhymeCard("gh", "ghế"), ["cá", "ghế", "bò", "mẹ"], 1),
    L!(140, Subject::TiengViet, "Chữ cái", "Bài 19. Ngh", "Từ nào có ngh?", Picture::RhymeCard("ngh", "nghề"), ["nghề", "bò", "cá", "lá"], 0),
    L!(141, Subject::TiengViet, "Chữ cái", "Bài 21. R", "Đâu là chữ R?", Picture::LetterCard("R", "r"), ["S", "T", "R", "P"], 2),
    L!(142, Subject::TiengViet, "Chữ cái", "Bài 22. Tr", "Từ nào có tr?", Picture::RhymeCard("tr", "trời"), ["cá", "bò", "mẹ", "trời"], 3),
    L!(143, Subject::TiengViet, "Chữ cái", "Bài 26. Ph", "Từ nào có ph?", Picture::RhymeCard("ph", "phố"), ["phố", "cá", "bò", "lá"], 0),
    L!(144, Subject::TiengViet, "Chữ cái", "Bài 27. X", "Đâu là chữ X?", Picture::LetterCard("X", "x"), ["K", "Y", "X", "Z"], 2),
    L!(145, Subject::TiengViet, "Vần", "Bài 33. en", "Từ nào có vần en?", Picture::RhymeCard("en", "bên"), ["bò", "bên", "mơ", "cá"], 1),
    L!(146, Subject::TiengViet, "Vần", "Bài 37. em", "Từ nào có vần em?", Picture::RhymeCard("em", "em bé"), ["bò", "cá", "lá", "em bé"], 3),
    L!(147, Subject::TiengViet, "Vần", "Bài 38. ay", "Từ nào có vần ay?", Picture::RhymeCard("ay", "may"), ["may", "bò", "mơ", "cá"], 0),
    L!(148, Subject::TiengViet, "Vần", "Bài 38. ây", "Từ nào có vần ây?", Picture::RhymeCard("ây", "bây"), ["ba", "bây", "bò", "mẹ"], 1),
    L!(149, Subject::TiengViet, "Vần", "Bài 39. ôi", "Từ nào có vần ôi?", Picture::RhymeCard("ôi", "khôi"), ["tôi", "ba", "cá", "khôi"], 3),
    L!(150, Subject::TiengViet, "Vần", "Bài 42. eo", "Từ nào có vần eo?", Picture::RhymeCard("eo", "reo"), ["mẹ", "bò", "reo", "cơm"], 2),
    L!(151, Subject::TiengViet, "Vần", "Bài 43. ều", "Từ nào có vần ều?", Picture::RhymeCard("ều", "diều"), ["diều", "ba", "câu", "bò"], 0),
    L!(152, Subject::TiengViet, "Vần tập 2", "Bài 44. iu", "Từ nào có vần iu?", Picture::RhymeCard("iu", "xiu"), ["ba", "xiu", "bò", "mẹ"], 1),
    L!(153, Subject::TiengViet, "Vần tập 2", "Bài 44. ưu", "Từ nào có vần ưu?", Picture::RhymeCard("ưu", "ưu tiên"), ["ba", "bò", "ưu tiên", "mẹ"], 2),
    L!(154, Subject::TiengViet, "Vần tập 2", "Bài 46. ăc", "Từ nào có vần ăc?", Picture::RhymeCard("ăc", "lắc"), ["lắc", "ba", "bò", "mẹ"], 0),
    L!(155, Subject::TiengViet, "Vần tập 2", "Bài 47. ôc", "Từ nào có vần ôc?", Picture::RhymeCard("ôc", "sốc"), ["ba", "bò", "mẹ", "sốc"], 3),
    L!(156, Subject::TiengViet, "Vần tập 2", "Bài 49. ôt", "Từ nào có vần ôt?", Picture::RhymeCard("ôt", "khót"), ["ba", "khót", "bò", "mẹ"], 1),
    L!(157, Subject::TiengViet, "Vần tập 2", "Bài 51. ết", "Từ nào có vần ết?", Picture::RhymeCard("ết", "kết"), ["ba", "bò", "kết", "mẹ"], 2),
    L!(158, Subject::TiengViet, "Vần tập 2", "Bài 52. ut", "Từ nào có vần ut?", Picture::RhymeCard("ut", "hút"), ["ba", "bò", "mẹ", "hút"], 3),
    L!(159, Subject::TiengViet, "Vần tập 2", "Bài 53. ắp", "Từ nào có vần ắp?", Picture::RhymeCard("ắp", "kẹp"), ["ba", "kẹp", "bò", "mẹ"], 1),
    L!(160, Subject::TiengViet, "Vần tập 2", "Bài 54. ợp", "Từ nào có vần ợp?", Picture::RhymeCard("ợp", "cặp"), ["cặp", "ba", "bò", "mẹ"], 0),
    L!(161, Subject::TiengViet, "Vần tập 2", "Bài 56. ẹp", "Từ nào có vần ẹp?", Picture::RhymeCard("ẹp", "vẹp"), ["ba", "bò", "vẹp", "mẹ"], 2),
    L!(162, Subject::TiengViet, "Vần tập 2", "Bài 57. inh", "Từ nào có vần inh?", Picture::RhymeCard("inh", "tinh"), ["ba", "bò", "mẹ", "tinh"], 3),
    L!(163, Subject::TiengViet, "Vần tập 2", "Bài 59. ăng", "Từ nào có vần ăng?", Picture::RhymeCard("ăng", "lăng"), ["lăng", "ba", "bò", "mẹ"], 0),
    L!(164, Subject::TiengViet, "Vần tập 2", "Bài 61. ung", "Từ nào có vần ung?", Picture::RhymeCard("ung", "chung"), ["ba", "bò", "chung", "mẹ"], 2),
    L!(165, Subject::TiengViet, "Vần tập 2", "Bài 62. iên", "Từ nào có vần iên?", Picture::RhymeCard("iên", "liên"), ["ba", "liên", "bò", "mẹ"], 1),
    L!(166, Subject::TiengViet, "Vần tập 2", "Bài 63. yên", "Từ nào có vần yên?", Picture::RhymeCard("yên", "yên bình"), ["ba", "bò", "mẹ", "yên bình"], 3),
    L!(167, Subject::TiengViet, "Vần tập 2", "Bài 66. uôi", "Từ nào có vần uôi?", Picture::RhymeCard("uôi", "nuôi"), ["nuôi", "ba", "bò", "mẹ"], 0),
    L!(168, Subject::TiengViet, "Vần tập 2", "Bài 67. uốc", "Từ nào có vần uốc?", Picture::RhymeCard("uốc", "muốc"), ["ba", "muốc", "bò", "mẹ"], 1),
    L!(169, Subject::TiengViet, "Vần tập 2", "Bài 68. uôn", "Từ nào có vần uôn?", Picture::RhymeCard("uôn", "khuôn"), ["ba", "bò", "mẹ", "khuôn"], 3),
    L!(170, Subject::TiengViet, "Vần tập 2", "Bài 74. oa", "Từ nào có vần oa?", Picture::RhymeCard("oa", "hoa"), ["ba", "bò", "hoa", "mẹ"], 2),
    L!(171, Subject::TiengViet, "Vần tập 2", "Bài 76. oan", "Từ nào có vần oan?", Picture::RhymeCard("oan", "xoan"), ["xoan", "ba", "bò", "mẹ"], 0),
    L!(172, Subject::TiengViet, "Vần tập 2", "Bài 77. oai", "Từ nào có vần oai?", Picture::RhymeCard("oai", "oai vệ"), ["ba", "oai vệ", "bò", "mẹ"], 1),
    L!(173, Subject::TiengViet, "Vần tập 2", "Bài 78. uân", "Từ nào có vần uân?", Picture::RhymeCard("uân", "khuân"), ["ba", "bò", "khuân", "mẹ"], 2),
    L!(174, Subject::TiengViet, "Vần tập 2", "Bài 79. uyên", "Từ nào có vần uyên?", Picture::RhymeCard("uyên", "nguyên"), ["nguyên", "ba", "bò", "mẹ"], 0),
    // Tiếng Anh 1 Global Success — 16 unit (thứ tự unit tăng dần)
    L!(95, Subject::TiengAnh, "Unit 1 Playground", "Phonics Bb", "Chữ nào có trong ball?", Picture::WordCard("B", "ball", "\u{26BD}"), ["B", "C", "D", "S"], 0),
    L!(96, Subject::TiengAnh, "Unit 1 Playground", "ball", "Quả bóng tiếng Anh là gì?", Picture::WordCard("B", "ball", "\u{26BD}"), ["book", "ball", "bike", "bell"], 1),
    L!(97, Subject::TiengAnh, "Unit 1 Playground", "Hi", "Khi gặp bạn Bill, mình nói gì?", Picture::WordCard("H", "Hi", "\u{1F4D6}"), ["Bye", "Stop", "Hi", "Sit"], 2),
    L!(98, Subject::TiengAnh, "Unit 1 Playground", "book", "Quyển sách tiếng Anh là gì?", Picture::WordCard("B", "book", "\u{1F4D6}"), ["ball", "bike", "bag", "book"], 3),
    L!(99, Subject::TiengAnh, "Unit 1 Playground", "bike", "Xe đạp tiếng Anh là gì?", Picture::WordCard("B", "bike", "\u{1F6B2}"), ["bus", "bike", "ball", "bag"], 1),
    L!(100, Subject::TiengAnh, "Unit 1 Playground", "Bye", "Khi chia tay, mình nói gì?", Picture::WordCard("B", "Bye", "\u{1F4D6}"), ["Bye", "Hi", "Sit", "Red"], 0),
    L!(101, Subject::TiengAnh, "Unit 1 Playground", "I have a ball", "“I have a ball” nghĩa gần nhất?", Picture::WordCard("B", "ball", "\u{26BD}"), ["Tôi có quả bóng", "Tôi có xe đạp", "Tôi thích sách", "Tạm biệt"], 0),
    L!(102, Subject::TiengAnh, "Unit 2 Dining room", "cat", "Con mèo tiếng Anh là gì?", Picture::WordCard("C", "cat", "\u{1F431}"), ["car", "cup", "cat", "cake"], 2),
    L!(103, Subject::TiengAnh, "Unit 2 Dining room", "I have a car", "“I have a car” nghĩa gần nhất?", Picture::WordCard("C", "car", "\u{1F697}"), ["Tôi ăn bánh", "Tôi ngủ", "Tôi chạy", "Tôi có ô tô"], 3),
    L!(104, Subject::TiengAnh, "Unit 2 Dining room", "I have a cup", "“I have a cup” nghĩa gần nhất?", Picture::WordCard("C", "cup", "\u{2615}"), ["Tôi có con mèo", "Tôi có cái cốc", "Tôi thích sữa", "Xin chào"], 1),
    L!(105, Subject::TiengAnh, "Unit 3 Market", "apple", "Quả táo tiếng Anh là gì?", Picture::WordCard("A", "apple", "\u{1F34E}"), ["apple", "hat", "bag", "can"], 0),
    L!(106, Subject::TiengAnh, "Unit 3 Market", "bag", "Cái túi tiếng Anh là gì?", Picture::WordCard("B", "bag", "\u{1F392}"), ["ball", "bus", "bag", "bed"], 2),
    L!(107, Subject::TiengAnh, "Unit 3 Market", "This is my bag", "“This is my bag” nghĩa gần nhất?", Picture::WordCard("B", "bag", "\u{1F392}"), ["Tôi thích sữa", "Đây là túi của tôi", "Xin chào", "Tạm biệt"], 1),
    L!(108, Subject::TiengAnh, "Unit 3 Market", "can", "Cái hộp tiếng Anh là gì?", Picture::WordCard("C", "can", "\u{1F96A}"), ["can", "hat", "bag", "pen"], 0),
    L!(109, Subject::TiengAnh, "Unit 4 Bedroom", "dog", "Con chó tiếng Anh là gì?", Picture::WordCard("D", "dog", "\u{1F436}"), ["duck", "desk", "door", "dog"], 3),
    L!(110, Subject::TiengAnh, "Unit 4 Bedroom", "door", "Cửa ra vào tiếng Anh là gì?", Picture::WordCard("D", "door", "\u{1F6AA}"), ["door", "desk", "dog", "duck"], 0),
    L!(111, Subject::TiengAnh, "Unit 4 Bedroom", "duck", "Con vịt tiếng Anh là gì?", Picture::WordCard("D", "duck", "\u{1F986}"), ["dog", "duck", "door", "desk"], 1),
    L!(112, Subject::TiengAnh, "Unit 4 Bedroom", "desk", "Cái bàn học tiếng Anh là gì?", Picture::WordCard("D", "desk", "\u{1F6CB}\u{FE0F}"), ["dog", "duck", "door", "desk"], 3),
    L!(113, Subject::TiengAnh, "Unit 5 Fish and chips", "milk", "Sữa tiếng Anh là gì?", Picture::WordCard("M", "milk", "\u{1F95B}"), ["fish", "milk", "chips", "chicken"], 1),
    L!(114, Subject::TiengAnh, "Unit 5 Fish and chips", "I like milk", "“I like milk” nghĩa gần nhất?", Picture::WordCard("M", "milk", "\u{1F95B}"), ["Tôi có ô tô", "Tôi chào", "Tôi ngủ", "Tôi thích sữa"], 3),
    L!(115, Subject::TiengAnh, "Unit 5 Fish and chips", "fish", "Con cá tiếng Anh là gì?", Picture::WordCard("F", "fish", "\u{1F41F}"), ["milk", "chips", "fish", "chicken"], 2),
    L!(116, Subject::TiengAnh, "Unit 5 Fish and chips", "chips", "Khoai chiên tiếng Anh là gì?", Picture::WordCard("C", "chips", "\u{1F35F}"), ["chips", "fish", "milk", "apple"], 0),
    L!(117, Subject::TiengAnh, "Unit 6 Classroom", "red", "Màu đỏ tiếng Anh là gì?", Picture::WordCard("R", "red", "\u{1F534}"), ["pen", "bell", "red", "desk"], 2),
    L!(118, Subject::TiengAnh, "Unit 6 Classroom", "pen", "Cái bút tiếng Anh là gì?", Picture::WordCard("P", "pen", "\u{1F58B}\u{FE0F}"), ["pen", "pencil", "bell", "book"], 0),
    L!(119, Subject::TiengAnh, "Unit 6 Classroom", "bell", "Cái chuông tiếng Anh là gì?", Picture::WordCard("B", "bell", "\u{1F514}"), ["pen", "bell", "red", "desk"], 1),
    L!(120, Subject::TiengAnh, "Unit 6 Classroom", "pencil", "Bút chì tiếng Anh là gì?", Picture::WordCard("P", "pencil", "\u{270F}\u{FE0F}"), ["pen", "bell", "pencil", "book"], 2),
    L!(121, Subject::TiengAnh, "Unit 6 Classroom", "This is my pen", "“This is my pen” nghĩa gần nhất?", Picture::WordCard("P", "pen", "\u{1F58B}\u{FE0F}"), ["Đây là sách của tôi", "Tôi có ô tô", "Đây là bút của tôi", "Tạm biệt"], 2),
    L!(122, Subject::TiengAnh, "Unit 7 Garden", "garden", "Khu vườn tiếng Anh là gì?", Picture::WordCard("G", "garden", "\u{1F337}"), ["gate", "garden", "goat", "girl"], 1),
    L!(123, Subject::TiengAnh, "Unit 7 Garden", "goat", "Con dê tiếng Anh là gì?", Picture::WordCard("G", "goat", "\u{1F410}"), ["girl", "gate", "goat", "garden"], 2),
    L!(124, Subject::TiengAnh, "Unit 7 Garden", "gate", "Cổng nhà tiếng Anh là gì?", Picture::WordCard("G", "gate", "\u{1F6AA}"), ["gate", "goat", "girl", "garden"], 0),
    L!(125, Subject::TiengAnh, "Unit 7 Garden", "girl", "Cô bé tiếng Anh là gì?", Picture::WordCard("G", "girl", "\u{1F467}"), ["goat", "gate", "garden", "girl"], 3),
    L!(126, Subject::TiengAnh, "Unit 8 Park", "hand", "Bàn tay tiếng Anh là gì?", Picture::WordCard("H", "hand", "\u{270B}"), ["hand", "head", "hair", "horse"], 0),
    L!(175, Subject::TiengAnh, "Unit 8 Park", "head", "Cái đầu tiếng Anh là gì?", Picture::WordCard("H", "head", "\u{1F60A}"), ["hand", "hat", "horse", "head"], 3),
    L!(176, Subject::TiengAnh, "Unit 8 Park", "hair", "Tóc tiếng Anh là gì?", Picture::WordCard("H", "hair", "\u{1F9B0}"), ["hand", "hair", "head", "horse"], 1),
    L!(177, Subject::TiengAnh, "Unit 8 Park", "horse", "Con ngựa tiếng Anh là gì?", Picture::WordCard("H", "horse", "\u{1F40E}"), ["hand", "hair", "horse", "head"], 2),
    L!(178, Subject::TiengAnh, "Unit 9 Shop", "clock", "Đồng hồ tiếng Anh là gì?", Picture::WordCard("C", "clock", "\u{1F550}"), ["lock", "clock", "cup", "car"], 1),
    L!(179, Subject::TiengAnh, "Unit 9 Shop", "five", "Số 5 tiếng Anh là gì?", Picture::WordCard("F", "five", "5\u{FE0F}\u{20E3}"), ["four", "nine", "five", "two"], 2),
    L!(180, Subject::TiengAnh, "Unit 9 Shop", "lock", "Cái khóa tiếng Anh là gì?", Picture::WordCard("L", "lock", "\u{1F512}"), ["clock", "cup", "car", "lock"], 3),
    L!(181, Subject::TiengAnh, "Unit 9 Shop", "cup", "Cái cốc tiếng Anh là gì?", Picture::WordCard("C", "cup", "\u{2615}"), ["lock", "cup", "clock", "car"], 1),
    L!(182, Subject::TiengAnh, "Unit 10 Zoo", "monkey", "Con khỉ tiếng Anh là gì?", Picture::WordCard("M", "monkey", "\u{1F412}"), ["mouse", "mango", "mother", "monkey"], 3),
    L!(183, Subject::TiengAnh, "Unit 10 Zoo", "At the zoo", "Ở sở thú mình gặp con gì?", Picture::WordCard("M", "monkey", "\u{1F993}"), ["desk", "monkey", "pen", "clock"], 1),
    L!(184, Subject::TiengAnh, "Unit 10 Zoo", "mouse", "Con chuột tiếng Anh là gì?", Picture::WordCard("M", "mouse", "\u{1F401}"), ["mouse", "monkey", "mango", "mother"], 0),
    L!(185, Subject::TiengAnh, "Unit 10 Zoo", "mango", "Quả xoài tiếng Anh là gì?", Picture::WordCard("M", "mango", "\u{1F96D}"), ["mouse", "monkey", "mango", "mother"], 2),
    L!(186, Subject::TiengAnh, "Unit 11 Bus stop", "bus", "Xe buýt tiếng Anh là gì?", Picture::WordCard("B", "bus", "\u{1F68C}"), ["bus", "bike", "ball", "bag"], 0),
    L!(187, Subject::TiengAnh, "Unit 11 Bus stop", "bus stop", "Điểm dừng xe buýt tiếng Anh là gì?", Picture::WordCard("B", "bus stop", "\u{1F68F}"), ["bike", "ball", "bag", "bus stop"], 3),
    L!(188, Subject::TiengAnh, "Unit 12 Lake", "lake", "Hồ nước tiếng Anh là gì?", Picture::WordCard("L", "lake", "\u{1F3DE}\u{FE0F}"), ["leaf", "lemon", "lake", "lock"], 2),
    L!(189, Subject::TiengAnh, "Unit 12 Lake", "leaf", "Chiếc lá tiếng Anh là gì?", Picture::WordCard("L", "leaf", "\u{1F343}"), ["leaf", "lake", "lemon", "lock"], 0),
    L!(190, Subject::TiengAnh, "Unit 12 Lake", "lemon", "Quả chanh tiếng Anh là gì?", Picture::WordCard("L", "lemon", "\u{1F34B}"), ["leaf", "lake", "lemon", "lock"], 2),
    L!(191, Subject::TiengAnh, "Unit 13 Canteen", "banana", "Quả chuối tiếng Anh là gì?", Picture::WordCard("B", "banana", "\u{1F34C}"), ["apple", "nut", "milk", "banana"], 3),
    L!(192, Subject::TiengAnh, "Unit 13 Canteen", "nut", "Quả hạt tiếng Anh là gì?", Picture::WordCard("N", "nut", "\u{1F330}"), ["banana", "nut", "milk", "apple"], 1),
    L!(193, Subject::TiengAnh, "Unit 14 Toy shop", "tiger", "Con hổ tiếng Anh là gì?", Picture::WordCard("T", "tiger", "\u{1F42F}"), ["tiger", "turtle", "teddy", "top"], 0),
    L!(194, Subject::TiengAnh, "Unit 14 Toy shop", "turtle", "Con rùa tiếng Anh là gì?", Picture::WordCard("T", "turtle", "\u{1F422}"), ["tiger", "teddy", "top", "turtle"], 3),
    L!(195, Subject::TiengAnh, "Unit 14 Toy shop", "teddy", "Gấu bông tiếng Anh là gì?", Picture::WordCard("T", "teddy", "\u{1F9F8}"), ["teddy", "turtle", "tiger", "top"], 0),
    L!(196, Subject::TiengAnh, "Unit 15 Football", "father", "Bố tiếng Anh là gì?", Picture::WordCard("F", "father", "\u{1F468}"), ["face", "foot", "father", "five"], 2),
    L!(197, Subject::TiengAnh, "Unit 15 Football", "football", "Bóng đá tiếng Anh là gì?", Picture::WordCard("F", "football", "\u{26BD}"), ["father", "football", "face", "five"], 1),
    L!(198, Subject::TiengAnh, "Unit 15 Football", "face", "Khuôn mặt tiếng Anh là gì?", Picture::WordCard("F", "face", "\u{1F600}"), ["football", "father", "five", "face"], 3),
    L!(199, Subject::TiengAnh, "Unit 16 At home", "water", "Nước tiếng Anh là gì?", Picture::WordCard("W", "water", "\u{1F4A7}"), ["window", "water", "wash", "Wendy"], 1),
    L!(200, Subject::TiengAnh, "Unit 16 At home", "window", "Cửa sổ tiếng Anh là gì?", Picture::WordCard("W", "window", "\u{1FA9F}"), ["door", "wall", "water", "window"], 3),
    L!(201, Subject::TiengAnh, "Unit 16 At home", "wall", "Bức tường tiếng Anh là gì?", Picture::WordCard("W", "wall", "\u{1F9F1}"), ["window", "water", "wall", "wash"], 2),
];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn scene_kinds_match_key_lessons() {
        assert!(matches!(by_id(49).unwrap().picture, Picture::Clock(3, 0)));
        assert!(matches!(
            by_id(36).unwrap().picture,
            Picture::Tomatoes(2, 3)
        ));
        assert!(matches!(by_id(39).unwrap().picture, Picture::Chart100(29)));
        assert!(matches!(
            by_id(15).unwrap().picture,
            Picture::FlatShape(FlatShape::Triangle)
        ));
        assert!(matches!(by_id(30).unwrap().picture, Picture::Ruler));
    }

    #[test]
    fn english_word_lessons_have_word_cards() {
        let n = for_subject(Subject::TiengAnh)
            .iter()
            .filter(|l| matches!(l.picture, Picture::WordCard(_, _, _)))
            .count();
        assert!(n >= 30, "word cards: {n}");
    }

    #[test]
    fn vietnamese_lessons_have_letter_or_rhyme_cards() {
        let n = for_subject(Subject::TiengViet)
            .iter()
            .take(32)
            .filter(|l| {
                matches!(
                    l.picture,
                    Picture::LetterCard(_, _) | Picture::RhymeCard(_, _)
                )
            })
            .count();
        assert!(n >= 31, "letter/rhyme cards: {n}");
    }

    #[test]
    fn comparison_lessons_have_two_groups() {
        for id in [5, 6, 7, 9, 19, 20] {
            assert!(
                matches!(
                    by_id(id).unwrap().picture,
                    Picture::Compare(_, _) | Picture::Join(_, _)
                ),
                "lesson {id}"
            );
        }
    }

    #[test]
    fn math_follows_sgk_length() {
        assert_eq!(for_subject(Subject::Toan).len(), 62);
        assert_eq!(for_subject(Subject::TiengViet).len(), 80);
        assert_eq!(for_subject(Subject::TiengAnh).len(), 59);
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
        let blob = format!(
            "{} {} {}",
            lesson.prompt,
            lesson.title,
            lesson.choices.join(" ")
        );
        assert!(!blob.contains("tay phải"));
        assert!(!blob.contains("tay trái"));
        assert!(lesson.prompt.contains("trái") || lesson.choices.contains(&"phải"));
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
