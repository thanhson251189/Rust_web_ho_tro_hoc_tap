use crate::lessons::{for_subject, Lesson, Picture, Subject};
use crate::store::Profile;

pub fn escape(input: &str) -> String {
    let mut out = String::with_capacity(input.len());
    for c in input.chars() {
        match c {
            '&' => out.push_str("&amp;"),
            '<' => out.push_str("&lt;"),
            '>' => out.push_str("&gt;"),
            '"' => out.push_str("&quot;"),
            _ => out.push(c),
        }
    }
    out
}

pub fn avatar_emoji(key: &str) -> &'static str {
    match key {
        "cat" => "🐱",
        "bear" => "🐻",
        "fox" => "🦊",
        _ => "🤖",
    }
}

pub fn page(title: &str, body: &str) -> String {
    format!(
        "<!DOCTYPE html><html lang=vi><head><meta charset=utf-8>\
<meta name=viewport content='width=device-width, initial-scale=1, viewport-fit=cover'>\
<title>{title}</title>\
<link rel=preconnect href='https://fonts.googleapis.com'>\
<link rel=preconnect href='https://fonts.gstatic.com' crossorigin>\
<link href='https://fonts.googleapis.com/css2?family=Be+Vietnam+Pro:wght@500;700;800&family=Fredoka:wght@500;600;700&display=swap' rel=stylesheet>\
<style>{css}</style></head><body><div class=shell>{body}</div></body></html>",
        title = escape(title),
        css = STYLES,
        body = body
    )
}

const STYLES: &str = r#"
:root {
  --ink: #3a2418;
  --muted: #8a6756;
  --sky: #b9e4f7;
  --sky2: #eaf7ff;
  --paper: #fff7ec;
  --sheet: #fffdf9;
  --ok: #1b7a45;
  --bad: #c43c2c;
  --toan: #ff8a4a;
  --viet: #3cbf78;
  --anh: #5b8cff;
}
* { box-sizing: border-box; }
html { -webkit-font-smoothing: antialiased; }
html, body { margin: 0; min-height: 100%; }
body {
  font-family: "Be Vietnam Pro", "Segoe UI", sans-serif;
  color: var(--ink);
  background:
    linear-gradient(180deg, var(--sky) 0 11rem, var(--sky2) 11rem 14rem, var(--paper) 14rem);
  line-height: 1.35;
}
.shell { width: min(52rem, calc(100% - 2rem)); margin: 0 auto; padding: 1.25rem 0 4rem; }
h1, h2, .display {
  font-family: Fredoka, "Trebuchet MS", sans-serif;
  letter-spacing: -0.03em;
  margin: 0;
}
h1 { font-size: clamp(2rem, 6vw, 3rem); font-weight: 700; text-wrap: balance; }
h2.unit {
  font-size: 1.15rem; color: var(--muted); margin: 1.4rem 0 .55rem;
  font-weight: 600;
}
.brand { font-family: Fredoka, sans-serif; font-weight: 700; font-size: 1rem; color: #2d6a8a; margin: 0 0 .6rem; }
.sub { color: var(--muted); font-size: 1.15rem; margin: .45rem 0 1.4rem; text-wrap: pretty; max-width: 36rem; }
.nav { margin: 0 0 1rem; }
.nav a {
  color: #2d6a8a; font-weight: 800; text-decoration: none;
  min-height: 2.75rem; display: inline-flex; align-items: center;
}
.faces { display: flex; gap: 1rem; flex-wrap: wrap; }
a.face {
  width: 8.5rem; text-decoration: none; color: inherit; text-align: center;
}
.face .bubble {
  width: 8.5rem; height: 8.5rem; border-radius: 50%;
  display: grid; place-items: center; font-size: 3.2rem;
  background: #fff; box-shadow: 0 10px 0 rgba(58,36,24,.08), 0 18px 28px rgba(58,36,24,.12);
  transition: transform .12s ease;
}
a.face:hover .bubble { transform: translateY(-4px); }
.face .name { display: block; margin-top: .65rem; font-family: Fredoka, sans-serif; font-size: 1.35rem; font-weight: 650; }
.hello { display: flex; gap: 1rem; align-items: center; margin: 0 0 1.2rem; }
.bubble.tiny {
  width: 4.5rem; height: 4.5rem; border-radius: 50%; flex: 0 0 auto;
  display: grid; place-items: center; font-size: 2.2rem; background: #fff;
  box-shadow: 0 8px 0 rgba(58,36,24,.08);
}
.worlds { display: flex; flex-direction: column; gap: 1rem; }
a.world {
  display: grid; grid-template-columns: minmax(4.5rem, 7.5rem) minmax(0, 1fr) auto; gap: 1rem; align-items: center;
  text-decoration: none; color: inherit; background: var(--sheet);
  border-radius: 1.75rem; padding: 1rem 1.2rem; min-height: 7.25rem;
  box-shadow: 0 12px 0 rgba(58,36,24,.07);
  overflow: hidden;
}
a.world:hover { transform: translateY(-2px); }
.world .art {
  width: 100%; max-width: 100%; height: 5.5rem; border-radius: 1.25rem;
  position: relative; overflow: hidden; justify-self: stretch;
}
.world .copy { min-width: 0; overflow: hidden; }
.world-toan .art { background: #ffd3b8; }
.world-viet .art { background: #c8f3d8; }
.world-anh .art { background: #cfe0ff; }
.world h2 {
  font-size: clamp(1.35rem, 5vw, 1.85rem);
  overflow-wrap: anywhere; word-break: break-word;
}
.world .hint {
  color: var(--muted); font-weight: 600; margin: .15rem 0 0;
  overflow-wrap: anywhere;
}
.go {
  font-family: Fredoka, sans-serif; font-weight: 700; background: #fff4b8;
  padding: .55rem .9rem; border-radius: 999px; white-space: nowrap;
}
.blob { position: absolute; border-radius: 50%; }
.world-toan .blob { width: 2.2rem; height: 2.2rem; background: #ff6b3d; top: 1.1rem; left: 1.1rem; }
.world-toan .blob.b { width: 1.5rem; height: 1.5rem; top: 2.6rem; left: 3.6rem; background: #ff9248; }
.world-viet .blob { width: 3.4rem; height: .7rem; border-radius: 8px; background: #2fa866; top: 1.4rem; left: 1.4rem; }
.world-viet .blob.b { width: 2.4rem; height: .7rem; top: 2.5rem; left: 1.4rem; background: #57c484; }
.world-anh .blob { width: 2.6rem; height: 1.4rem; border-radius: 1rem 1rem 0 0; background: #fff; top: 2.2rem; left: 2.2rem; }
.world-anh .blob.b { width: 1.1rem; height: 1.1rem; background: #ffd24a; top: .7rem; left: 4.4rem; }
form.stack { margin-top: 1.6rem; display: flex; gap: .8rem; flex-wrap: wrap; align-items: end; }
label { display: flex; flex-direction: column; gap: .35rem; font-weight: 700; }
input, select {
  font: inherit; font-size: 1.2rem; padding: .8rem 1rem; min-height: 3rem;
  border: 0; border-radius: 1rem; background: #fff;
  box-shadow: 0 0 0 2px #efd8c4 inset;
}
.btn, button[type=submit] {
  font-family: Fredoka, sans-serif; font-weight: 700; font-size: 1.15rem; cursor: pointer;
  border: 0; border-radius: 999px; padding: .85rem 1.3rem; min-height: 3rem;
  background: #ffd24a; color: var(--ink); text-decoration: none;
  display: inline-flex; align-items: center; box-shadow: 0 6px 0 #e0b21f;
}
.btn:hover, button[type=submit]:hover { transform: translateY(-1px); }
.error, .banner {
  font-size: 1.2rem; font-weight: 800; padding: 1rem 1.15rem; border-radius: 1.2rem; margin: 1rem 0;
}
.error, .bad { color: var(--bad); background: #ffe1db; }
.ok { color: var(--ok); background: #d7f5e3; }
.sheet {
  background: var(--sheet); border-radius: 1.8rem; padding: 1.25rem 1.25rem 1.5rem;
  box-shadow: 0 16px 0 rgba(58,36,24,.06);
}
.prompt {
  font-family: Fredoka, sans-serif; font-size: clamp(1.55rem, 4.5vw, 2.15rem);
  font-weight: 650; margin: .4rem 0 0; text-wrap: pretty;
}
.pic {
  display: flex; gap: .55rem; flex-wrap: wrap; min-height: 4.2rem; align-items: center;
  margin: .2rem 0 1rem; padding: .9rem; background: #fff6e3; border-radius: 1.2rem;
}
.pic.empty { color: var(--muted); font-weight: 700; }
.star {
  width: 2.3rem; height: 2.3rem; background: #ffc107;
  clip-path: polygon(50% 0%, 61% 35%, 98% 35%, 68% 57%, 79% 91%, 50% 70%, 21% 91%, 32% 57%, 2% 35%, 39% 35%);
}
.apple {
  width: 2.15rem; height: 2.35rem; background: #e53935; border-radius: 50% 50% 45% 45%;
  position: relative;
}
.apple::before {
  content: ""; position: absolute; width: .55rem; height: .7rem; background: #3d8b47;
  border-radius: 4px; top: -.35rem; left: 1.05rem; transform: rotate(20deg);
}
.dot { width: 1.7rem; height: 1.7rem; border-radius: 50%; background: #5b8cff; }
.block { width: 1.85rem; height: 1.85rem; border-radius: .45rem; background: #ff8a4a; }
.choices { display: grid; grid-template-columns: 1fr 1fr; gap: .85rem; margin-top: 1.15rem; }
button.choice {
  min-height: 4.6rem; font-size: clamp(1.35rem, 4vw, 1.7rem); width: 100%;
  font-family: Fredoka, sans-serif; font-weight: 700; border: 0; border-radius: 1.25rem;
  background: #fff; color: var(--ink); cursor: pointer;
  box-shadow: 0 0 0 3px #efe0d2 inset, 0 8px 0 rgba(58,36,24,.08);
}
.choices form:nth-child(1) button { background: #ffe8c8; }
.choices form:nth-child(2) button { background: #d9f4e4; }
.choices form:nth-child(3) button { background: #dce8ff; }
.choices form:nth-child(4) button { background: #ffe0ee; }
button.choice:hover { transform: translateY(-2px); }
.progress {
  display: flex; gap: .35rem; flex-wrap: wrap; margin: 0 0 .9rem; align-items: center;
  font-weight: 800; color: var(--muted); font-variant-numeric: tabular-nums;
}
.progress .tick {
  width: .7rem; height: .7rem; border-radius: 999px; background: #e8d4c4;
}
.progress .tick.on { background: #ffd24a; transform: scale(1.25); }
.lessons { display: flex; flex-direction: column; gap: .55rem; }
a.item {
  display: flex; justify-content: space-between; align-items: center; gap: 1rem;
  text-decoration: none; color: inherit; background: var(--sheet);
  border-radius: 1.15rem; padding: .95rem 1.1rem; min-height: 3.6rem;
  box-shadow: 0 8px 0 rgba(58,36,24,.05); font-weight: 800;
}
a.item span.n { color: var(--muted); font-variant-numeric: tabular-nums; }
.cta-row { margin: 0 0 1rem; }
.done-actions { display: flex; gap: .7rem; flex-wrap: wrap; }
a.world:focus-visible, a.face:focus-visible, button:focus-visible, .btn:focus-visible, input:focus-visible, select:focus-visible, a.item:focus-visible {
  outline: 3px solid #5b8cff; outline-offset: 3px;
}
@media (max-width: 640px) {
  .choices { grid-template-columns: 1fr; }
  a.world { grid-template-columns: minmax(3.6rem, 4.8rem) minmax(0, 1fr); gap: .75rem; padding: .9rem 1rem; }
  a.world .go { display: none; }
  .world .art { height: 4.2rem; }
}
@media (prefers-reduced-motion: reduce) {
  *, *::before, *::after { transition-duration: 0.01ms !important; }
}
"#;

fn picture_html(picture: Picture) -> String {
    let (kind, count) = match picture {
        Picture::None => return String::new(),
        Picture::Stars(n) => ("star", n),
        Picture::Apples(n) => ("apple", n),
        Picture::Dots(n) => ("dot", n),
        Picture::Blocks(n) => ("block", n),
    };
    if count == 0 {
        return "<div class='pic empty'>Không có gì cả.</div>".into();
    }
    let mut inner = String::new();
    for _ in 0..count {
        inner.push_str(&format!("<span class={kind}></span>"));
    }
    format!("<div class=pic aria-hidden=true>{inner}</div>")
}

pub fn picker(profiles: &[Profile], error: Option<&str>, can_add: bool) -> String {
    let mut cards = String::new();
    for profile in profiles {
        cards.push_str(&format!(
            "<a class=face href=/profiles/{id}><span class=bubble>{emoji}</span><span class=name>{name}</span></a>",
            id = profile.id,
            emoji = avatar_emoji(&profile.avatar_key),
            name = escape(&profile.name)
        ));
    }
    let form = if can_add {
        "<form class=stack method=post action=/profiles>\
<label>Tên bé <input name=name required maxlength=24 placeholder='Bé An'></label>\
<label>Hình <select name=avatar_key>\
<option value=robot>Robot</option><option value=cat>Mèo</option>\
<option value=bear>Gấu</option><option value=fox>Cáo</option>\
</select></label><button type=submit>Thêm hồ sơ</button></form>"
            .to_string()
    } else {
        String::new()
    };
    let err = error
        .map(|e| format!("<p class=error>{}</p>", escape(e)))
        .unwrap_or_default();
    page(
        "Ai đang học?",
        &format!(
            "<p class=brand>Học cùng bé</p><h1>Ai đang học?</h1>\
<p class=sub>Chạm vào hình của bé. Chữ to, nút to, một câu hỏi một lần.</p>\
<div class=faces>{cards}</div>{err}{form}"
        ),
    )
}

pub fn home(profile: &Profile) -> String {
    let mut worlds = String::new();
    for subject in Subject::all() {
        let class = match subject {
            Subject::Toan => "world-toan",
            Subject::TiengViet => "world-viet",
            Subject::TiengAnh => "world-anh",
        };
        let n = for_subject(subject).len();
        worlds.push_str(&format!(
            "<a class='world {class}' href=/profiles/{id}/mon/{slug}>\
<span class=art><span class=blob></span><span class='blob b'></span></span>\
<span class=copy><h2>{title}</h2><p class=hint>{n} bài · {hint}</p></span>\
<span class=go>Vào học</span></a>",
            id = profile.id,
            slug = subject.slug(),
            title = subject.title(),
            hint = subject.hint(),
            n = n,
        ));
    }
    page(
        "Học nào",
        &format!(
            "<div class=nav><a href=/profiles>← Đổi hồ sơ</a></div>\
<div class=hello><span class=bubble tiny>{emoji}</span>\
<div><h1>Xin chào, {name}!</h1>\
<p class=sub>Chọn một phòng học. Bé làm từng bài, không cần đọc chữ nhỏ.</p></div></div>\
<div class=worlds>{worlds}</div>",
            name = escape(&profile.name),
            emoji = avatar_emoji(&profile.avatar_key),
        ),
    )
}

pub fn subject_page(profile: &Profile, subject: Subject, lessons: &[&Lesson]) -> String {
    let mut list = String::new();
    let mut last_unit = "";
    for (index, lesson) in lessons.iter().enumerate() {
        if lesson.unit != last_unit {
            list.push_str(&format!("<h2 class=unit>{}</h2>", escape(lesson.unit)));
            last_unit = lesson.unit;
        }
        list.push_str(&format!(
            "<a class=item href=/profiles/{pid}/bai/{lid}><span>{title}</span><span class=n>Bài {n}</span></a>",
            pid = profile.id,
            lid = lesson.id,
            n = index + 1,
            title = escape(lesson.title),
        ));
    }
    let start = lessons
        .first()
        .map(|lesson| {
            format!(
                "<p class=cta-row><a class=btn href=/profiles/{}/bai/{}>Bắt đầu bài 1</a></p>",
                profile.id, lesson.id
            )
        })
        .unwrap_or_default();
    page(
        subject.title(),
        &format!(
            "<div class=nav><a href=/profiles/{pid}>Môn học</a></div>\
<h1>{title}</h1><p class=sub>Lớp 1 · {hint} · {count} bài</p>\
{start}<div class=lessons>{list}</div>",
            pid = profile.id,
            title = subject.title(),
            hint = subject.hint(),
            count = lessons.len(),
        ),
    )
}

pub fn missing() -> String {
    page(
        "Không tìm thấy",
        "<h1>Không có trang này.</h1><p class=sub><a href=/profiles>Về chọn hồ sơ</a></p>",
    )
}

pub fn missing_profile() -> String {
    page(
        "Không có hồ sơ",
        "<h1>Không có hồ sơ này.</h1><p class=sub><a href=/profiles>Quay lại chọn hồ sơ</a></p>",
    )
}

pub fn missing_subject() -> String {
    page(
        "Không có môn",
        "<h1>Không có môn này.</h1><p class=sub><a href=/profiles>Về chọn hồ sơ</a></p>",
    )
}

pub fn lesson_page(
    profile: &Profile,
    lesson: &Lesson,
    index: usize,
    total: usize,
    flash: Option<Flash>,
) -> String {
    let solved = matches!(flash, Some(Flash::Correct { .. }));
    let mut choices = String::new();
    if !solved {
        for (i, choice) in lesson.choices.iter().enumerate() {
            choices.push_str(&format!(
                "<form method=post action=/profiles/{pid}/bai/{lid}>\
<input type=hidden name=choice value={i}>\
<button class=choice type=submit>{label}</button></form>",
                pid = profile.id,
                lid = lesson.id,
                i = i,
                label = escape(choice),
            ));
        }
    }
    let banner = match flash {
        Some(Flash::Correct { next_id }) => {
            let next = match next_id {
                Some(id) => format!(
                    "<a class=btn href=/profiles/{}/bai/{id}>Bài tiếp theo</a>",
                    profile.id
                ),
                None => format!(
                    "<a class=btn href=/profiles/{}/mon/{}>Xong môn này</a>",
                    profile.id,
                    lesson.subject.slug()
                ),
            };
            format!(
                "<p class='banner ok'>Giỏi quá! Đúng rồi.</p><div class=done-actions>{next}\
<a class=btn href=/profiles/{pid}>Về chọn môn</a></div>",
                pid = profile.id,
                next = next
            )
        }
        Some(Flash::Wrong) => {
            "<p class='banner bad'>Chưa đúng. Thử lại nhé, bé ơi!</p>".to_string()
        }
        None => String::new(),
    };
    let mut dots = String::new();
    for i in 0..total {
        let on = if i == index { " on" } else { "" };
        dots.push_str(&format!("<span class='tick{on}'></span>"));
    }
    page(
        lesson.title,
        &format!(
            "<div class=nav><a href=/profiles/{pid}/mon/{slug}>{subject}</a></div>\
<div class=progress aria-label='Bài {n} trên {total}'>{dots}<span>Bài {n}/{total}</span></div>\
<div class=sheet><p class=sub>{unit} · {title}</p>\
{picture}<p class=prompt>{prompt}</p>{banner}<div class=choices>{choices}</div></div>",
            pid = profile.id,
            slug = lesson.subject.slug(),
            subject = lesson.subject.title(),
            n = index + 1,
            total = total,
            dots = dots,
            unit = escape(lesson.unit),
            title = escape(lesson.title),
            picture = picture_html(lesson.picture),
            prompt = escape(lesson.prompt),
        ),
    )
}

#[derive(Debug, Clone, Copy)]
pub enum Flash {
    Correct { next_id: Option<u32> },
    Wrong,
}
