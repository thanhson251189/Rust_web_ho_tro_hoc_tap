use crate::lessons::{Lesson, Subject};
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
<title>{title}</title><style>{css}</style></head><body>{body}</body></html>",
        title = escape(title),
        css = STYLES,
        body = body
    )
}

const STYLES: &str = r#"
:root {
  --ink: #2b1b12;
  --muted: #7a5c4a;
  --paper: #fff6e8;
  --card: #fffdf8;
  --ok: #1f8a4c;
  --bad: #c62828;
  --focus: #5b3cff;
}
* { box-sizing: border-box; }
html { -webkit-font-smoothing: antialiased; }
html, body { margin: 0; min-height: 100%; }
body {
  font-family: "Segoe UI", "Nunito", "Trebuchet MS", sans-serif;
  background:
    radial-gradient(circle at 12% 8%, #ffe08a 0 8%, transparent 9%),
    radial-gradient(circle at 88% 12%, #ffb4a2 0 7%, transparent 8%),
    radial-gradient(circle at 80% 86%, #a6e3c3 0 9%, transparent 10%),
    var(--paper);
  color: var(--ink);
  padding: 1.25rem 1.25rem 3rem;
  line-height: 1.35;
}
h1 {
  font-size: clamp(1.9rem, 5.5vw, 2.7rem);
  margin: 0 0 .45rem;
  letter-spacing: -0.03em;
  text-wrap: balance;
}
.sub { color: var(--muted); font-size: 1.2rem; margin: 0 0 1.5rem; text-wrap: pretty; }
.row { display: flex; gap: 1rem; flex-wrap: wrap; }
.grid { display: grid; grid-template-columns: repeat(auto-fit, minmax(220px, 1fr)); gap: 1rem; }
a.card, button.choice, .card-static {
  display: flex; flex-direction: column; align-items: center; justify-content: center;
  text-align: center; text-decoration: none; color: inherit;
  background: var(--card); border: 3px solid var(--ink); border-radius: 1.5rem;
  padding: 1.4rem 1rem; min-height: 9.5rem; min-width: 8.5rem;
  box-shadow: 6px 6px 0 var(--ink); font-weight: 800;
  transition: transform .12s cubic-bezier(0.2, 0, 0, 1), box-shadow .12s cubic-bezier(0.2, 0, 0, 1);
}
a.card:hover, button.choice:hover { transform: translate(-2px, -2px); box-shadow: 8px 8px 0 var(--ink); }
a.card:active, button.choice:active { transform: scale(0.96) translate(2px, 2px); box-shadow: 2px 2px 0 var(--ink); }
a.card:focus-visible, button.choice:focus-visible, .btn:focus-visible, button:focus-visible, input:focus-visible, select:focus-visible {
  outline: 3px solid var(--focus); outline-offset: 3px;
}
.emoji { font-size: 3rem; line-height: 1; }
.name { font-size: 1.5rem; margin-top: .55rem; }
.hint { color: var(--muted); font-size: 1.05rem; font-weight: 700; margin-top: .25rem; }
.subject-toan { background: #ffe1d2; }
.subject-viet { background: #d9f7e5; }
.subject-anh { background: #dce8ff; }
form.stack { margin-top: 1.4rem; display: flex; gap: .8rem; flex-wrap: wrap; align-items: end; }
label { display: flex; flex-direction: column; gap: .35rem; font-weight: 700; font-size: 1.05rem; }
input, select {
  font: inherit; font-size: 1.25rem; padding: .85rem 1rem; min-height: 3rem;
  border: 3px solid var(--ink); border-radius: 1rem; background: #fff;
}
button, .btn {
  font: inherit; font-weight: 800; font-size: 1.2rem; cursor: pointer;
  border: 3px solid var(--ink); border-radius: 1.1rem; padding: .85rem 1.25rem;
  min-height: 3rem; background: #ffd54a; color: var(--ink); box-shadow: 4px 4px 0 var(--ink);
  text-decoration: none; display: inline-flex; align-items: center; justify-content: center; gap: .4rem;
  transition: transform .12s cubic-bezier(0.2, 0, 0, 1), box-shadow .12s cubic-bezier(0.2, 0, 0, 1);
}
.btn:hover, button:hover { transform: translate(-1px, -1px); }
.btn:active, button:active { transform: scale(0.96); }
.error, .banner {
  font-size: 1.25rem; font-weight: 800; padding: 1rem 1.1rem; border-radius: 1.1rem;
  border: 3px solid var(--ink); box-shadow: 4px 4px 0 var(--ink); margin: 1rem 0;
}
.error { color: var(--bad); background: #ffd6d6; }
.ok { color: var(--ok); background: #d6f5e2; }
.bad { color: var(--bad); background: #ffd6d6; }
.choices { display: grid; grid-template-columns: 1fr 1fr; gap: 1rem; margin-top: 1.25rem; }
button.choice {
  min-height: 5.75rem; font-size: clamp(1.35rem, 4vw, 1.7rem); width: 100%;
  background: #fff; font-variant-numeric: tabular-nums;
}
.nav { margin: .2rem 0 1.2rem; display: flex; gap: .9rem; flex-wrap: wrap; align-items: center; }
.nav a {
  color: var(--ink); font-weight: 800; font-size: 1.1rem;
  min-height: 2.75rem; display: inline-flex; align-items: center;
  padding: .35rem .55rem; border-radius: .75rem;
}
.prompt {
  font-size: clamp(1.55rem, 4.5vw, 2.25rem); font-weight: 800; margin: .5rem 0 0;
  text-wrap: pretty; background: #fff; border: 3px solid var(--ink); border-radius: 1.25rem;
  padding: 1.1rem 1.2rem; box-shadow: 5px 5px 0 var(--ink);
}
.lesson-list a.card { min-height: 6.75rem; align-items: flex-start; text-align: left; padding: 1.2rem 1.15rem; }
.lesson-list .name { margin-top: 0; font-size: 1.35rem; }
.progress {
  display: flex; gap: .45rem; flex-wrap: wrap; margin: 0 0 1rem; align-items: center;
  font-variant-numeric: tabular-nums; font-weight: 800; color: var(--muted);
}
.progress .dot {
  width: .85rem; height: .85rem; border-radius: 999px; border: 2px solid var(--ink);
  background: #fff;
}
.progress .dot.on { background: #ffd54a; }
.cta-row { margin: 0 0 1.25rem; }
.done-actions { display: flex; gap: .8rem; flex-wrap: wrap; margin-top: .6rem; }
@media (max-width: 640px) {
  body { padding: 1rem 1rem 2.5rem; }
  .choices { grid-template-columns: 1fr; }
  a.card { min-height: 8rem; width: 100%; }
}
@media (prefers-reduced-motion: reduce) {
  *, *::before, *::after {
    transition-duration: 0.01ms !important;
  }
}
"#;

pub fn picker(profiles: &[Profile], error: Option<&str>, can_add: bool) -> String {
    let mut cards = String::new();
    for profile in profiles {
        cards.push_str(&format!(
            "<a class=card href=/profiles/{id}><span class=emoji>{emoji}</span><span class=name>{name}</span></a>",
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
            "<h1>Ai đang học?</h1><p class=sub>Chạm vào hình của bé để bắt đầu.</p>\
<div class=row>{cards}</div>{err}{form}"
        ),
    )
}

pub fn home(profile: &Profile) -> String {
    let mut cards = String::new();
    for subject in Subject::all() {
        let class = match subject {
            Subject::Toan => "subject-toan",
            Subject::TiengViet => "subject-viet",
            Subject::TiengAnh => "subject-anh",
        };
        cards.push_str(&format!(
            "<a class='card {class}' href=/profiles/{id}/mon/{slug}>\
<span class=emoji>{emoji}</span><span class=name>{title}</span>\
<span class=hint>{hint}</span></a>",
            id = profile.id,
            slug = subject.slug(),
            emoji = subject.emoji(),
            title = subject.title(),
            hint = subject.hint(),
        ));
    }
    page(
        "Học nào",
        &format!(
            "<div class=nav><a href=/profiles>← Đổi hồ sơ</a></div>\
<h1>Xin chào, {name}! {emoji}</h1>\
<p class=sub>Chọn một môn nhé. Bài ngắn, chữ to, chạm là trả lời.</p>\
<div class=grid>{cards}</div>",
            name = escape(&profile.name),
            emoji = avatar_emoji(&profile.avatar_key),
        ),
    )
}

pub fn subject_page(profile: &Profile, subject: Subject, lessons: &[&Lesson]) -> String {
    let mut list = String::new();
    for (index, lesson) in lessons.iter().enumerate() {
        list.push_str(&format!(
            "<a class=card href=/profiles/{pid}/bai/{lid}>\
<span class=emoji>⭐</span><span class=name>Bài {n}. {title}</span>\
<span class=hint>Chạm để làm</span></a>",
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
                "<p class=cta-row><a class=btn href=/profiles/{}/bai/{}>Bắt đầu bài 1 ▶</a></p>",
                profile.id, lesson.id
            )
        })
        .unwrap_or_default();
    page(
        subject.title(),
        &format!(
            "<div class=nav><a href=/profiles/{pid}>← Môn học</a></div>\
<h1>{emoji} {title}</h1><p class=sub>Lớp 1 · {hint} · {count} bài ngắn</p>\
{start}<div class='grid lesson-list'>{list}</div>",
            pid = profile.id,
            emoji = subject.emoji(),
            title = subject.title(),
            hint = subject.hint(),
            count = lessons.len(),
            start = start,
        ),
    )
}

pub fn missing() -> String {
    page(
        "Không tìm thấy",
        "<h1>Không có trang này.</h1><p class=sub><a href=/profiles>Về chọn hồ sơ</a></p>",
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
                    "<a class=btn href=/profiles/{}/bai/{id}>Bài tiếp theo ▶</a>",
                    profile.id
                ),
                None => format!(
                    "<a class=btn href=/profiles/{}/mon/{}>🎉 Xong môn này</a>",
                    profile.id,
                    lesson.subject.slug()
                ),
            };
            format!(
                "<p class='banner ok'>🌟 Giỏi quá! Đúng rồi.</p><div class=done-actions>{next}\
<a class=btn href=/profiles/{pid}>Về chọn môn</a></div>",
                pid = profile.id,
                next = next
            )
        }
        Some(Flash::Wrong) => {
            "<p class='banner bad'>😅 Chưa đúng. Thử lại nhé, bé ơi!</p>".to_string()
        }
        None => String::new(),
    };

    let mut dots = String::new();
    for i in 0..total {
        let on = if i == index { " on" } else { "" };
        dots.push_str(&format!("<span class='dot{on}' aria-hidden=true></span>"));
    }

    page(
        lesson.title,
        &format!(
            "<div class=nav><a href=/profiles/{pid}/mon/{slug}>← {subject}</a></div>\
<div class=progress aria-label='Tiến độ bài {n} trên {total}'>{dots}<span>Bài {n}/{total}</span></div>\
<p class=sub>{title}</p>\
<p class=prompt>{prompt}</p>{banner}<div class=choices>{choices}</div>",
            pid = profile.id,
            slug = lesson.subject.slug(),
            subject = lesson.subject.title(),
            n = index + 1,
            total = total,
            dots = dots,
            title = escape(lesson.title),
            prompt = escape(lesson.prompt),
        ),
    )
}

#[derive(Debug, Clone, Copy)]
pub enum Flash {
    Correct { next_id: Option<u32> },
    Wrong,
}
