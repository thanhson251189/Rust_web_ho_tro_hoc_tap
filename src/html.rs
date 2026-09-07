use crate::lessons::{for_subject, FlatShape, Lesson, Picture, SolidShape, Subject};
use crate::store::{DayStat, Profile};

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
<link href='https://fonts.googleapis.com/css2?family=Baloo+2:wght@500;600;700;800&family=Nunito:ital,wght@0,400;0,600;0,700;0,800;1,600&display=swap' rel=stylesheet>\
<style>{css}</style></head><body><div class=shell>{body}</div>\
<script>{js}</script></body></html>",
        title = escape(title),
        css = STYLES,
        js = SCRIPT,
        body = body
    )
}

const STYLES: &str = r#"
:root {
  --ink: #1f2a44;
  --muted: #6b7a99;
  --line: #e6ebf7;
  --paper: #f4f7ff;
  --sheet: #ffffff;
  --ok: #0e9f6e;
  --bad: #e02424;
  --toan: #ff8a3d;
  --toan-soft: #ffefe2;
  --viet: #2bb673;
  --viet-soft: #e2f7ec;
  --anh: #4f7cff;
  --anh-soft: #e7edff;
  --sun: #ffc531;
  --radius: 1.4rem;
}
* { box-sizing: border-box; }
html { -webkit-font-smoothing: antialiased; }
html, body { margin: 0; min-height: 100%; }
body {
  font-family: Nunito, "Segoe UI", sans-serif;
  font-size: 1.05rem;
  color: var(--ink);
  background: var(--paper);
  line-height: 1.5;
}
.shell { width: min(54rem, calc(100% - 2rem)); margin: 0 auto; padding: 1.1rem 0 4rem; }
h1, h2, .display, .btn, .go, .choice-word, .chip {
  font-family: "Baloo 2", "Trebuchet MS", sans-serif;
}
h1 {
  font-size: clamp(1.9rem, 5.5vw, 2.7rem); font-weight: 800;
  letter-spacing: -0.02em; margin: 0; text-wrap: balance;
}
h2.unit {
  font-size: 1.02rem; color: var(--muted); margin: 1.5rem 0 .6rem;
  font-weight: 800; text-transform: uppercase; letter-spacing: .06em;
}
.brand { color: var(--toan); font-weight: 800; letter-spacing: .02em; margin: 0 0 .5rem; }
.sub { color: var(--muted); font-size: 1.12rem; margin: .5rem 0 1.4rem; text-wrap: pretty; max-width: 38rem; }
.nav { margin: 0 0 1.1rem; }
.nav a {
  color: var(--muted); font-weight: 800; text-decoration: none;
  min-height: 2.75rem; display: inline-flex; align-items: center; gap: .3rem;
}
.nav a:hover { color: var(--toan); }

/* profile picker */
.faces { display: flex; gap: 1.1rem; flex-wrap: wrap; }
a.face { width: 8.6rem; text-decoration: none; color: inherit; text-align: center; }
.face .bubble {
  width: 8.6rem; height: 8.6rem; border-radius: 38% 62% 55% 45% / 50% 42% 58% 50%;
  display: grid; place-items: center; font-size: 3.4rem;
  background: linear-gradient(160deg, #fff 0 55%, var(--toan-soft));
  border: 3px solid var(--line);
  box-shadow: 0 14px 24px rgba(31,42,68,.10);
  transition: transform .15s ease, box-shadow .15s ease;
}
a.face:nth-child(2) .bubble { background: linear-gradient(160deg, #fff 0 55%, var(--viet-soft)); }
a.face:nth-child(3) .bubble { background: linear-gradient(160deg, #fff 0 55%, var(--anh-soft)); }
a.face:hover .bubble { transform: translateY(-5px) rotate(-2deg); }
.face .name { display: block; margin-top: .6rem; font-family: "Baloo 2", sans-serif; font-size: 1.3rem; font-weight: 700; }
.hello { display: flex; gap: 1.1rem; align-items: center; margin: 0 0 1.3rem; }
.bubble.tiny {
  width: 4.6rem; height: 4.6rem; flex: 0 0 auto; border-radius: 36% 64% 58% 42% / 52% 44% 56% 48%;
  display: grid; place-items: center; font-size: 2.3rem;
  background: linear-gradient(160deg, #fff 0 55%, var(--toan-soft));
  border: 3px solid var(--line); box-shadow: 0 10px 18px rgba(31,42,68,.10);
}

/* subject worlds */
.worlds { display: flex; flex-direction: column; gap: 1rem; }
a.world {
  display: grid; grid-template-columns: minmax(4.6rem, 7.6rem) minmax(0, 1fr) auto;
  gap: 1rem; align-items: center; text-decoration: none; color: inherit;
  background: var(--sheet); border-radius: var(--radius); padding: 1rem 1.2rem;
  min-height: 7.4rem; box-shadow: 0 10px 24px rgba(31,42,68,.07);
  border: 2px solid transparent; overflow: hidden;
}
a.world:hover { transform: translateY(-2px); border-color: var(--line); }
.world .art { width: 100%; height: 5.6rem; border-radius: 1.2rem; position: relative; overflow: hidden; }
.world .copy { min-width: 0; overflow: hidden; }
.world-toan .art { background: var(--toan-soft); }
.world-viet .art { background: var(--viet-soft); }
.world-anh .art { background: var(--anh-soft); }
.world h2 { font-size: clamp(1.3rem, 5vw, 1.8rem); margin: 0; overflow-wrap: anywhere; }
.world .hint { color: var(--muted); font-weight: 700; margin: .15rem 0 0; overflow-wrap: anywhere; }
.go {
  font-weight: 700; font-size: 1.05rem; background: var(--sun); color: #533f00;
  padding: .55rem 1rem; border-radius: 999px; white-space: nowrap;
  box-shadow: 0 4px 0 #d9a416;
}
.blob { position: absolute; border-radius: 50%; }
.world-toan .blob { width: 2.4rem; height: 2.4rem; background: var(--toan); top: 1rem; left: 1.1rem; }
.world-toan .blob.b { width: 1.4rem; height: 1.4rem; top: 2.8rem; left: 3.8rem; background: #ffb37d; }
.world-viet .blob { width: 3.6rem; height: .75rem; border-radius: 8px; background: var(--viet); top: 1.3rem; left: 1.2rem; }
.world-viet .blob.b { width: 2.4rem; height: .75rem; top: 2.5rem; left: 1.2rem; background: #7fdcac; }
.world-anh .blob { width: 2.7rem; height: 1.5rem; border-radius: 1rem 1rem 0 0; background: var(--anh); top: 2.1rem; left: 2rem; }
.world-anh .blob.b { width: 1.1rem; height: 1.1rem; background: var(--sun); top: .8rem; left: 4.4rem; }

form.stack { margin-top: 1.6rem; display: flex; gap: .8rem; flex-wrap: wrap; align-items: end; }
label { display: flex; flex-direction: column; gap: .35rem; font-weight: 800; }
input, select {
  font: inherit; font-size: 1.15rem; padding: .8rem 1rem; min-height: 3rem;
  border: 2px solid var(--line); border-radius: 1rem; background: #fff; color: var(--ink);
}
input:focus, select:focus { border-color: var(--anh); outline: none; }
.btn, button[type=submit] {
  font-weight: 700; font-size: 1.12rem; cursor: pointer; border: 0;
  border-radius: 999px; padding: .8rem 1.4rem; min-height: 3rem;
  background: var(--toan); color: #fff; text-decoration: none;
  display: inline-flex; align-items: center; gap: .4rem;
  box-shadow: 0 5px 0 #d96a1f;
  transition: transform .12s ease, box-shadow .12s ease, background .12s ease;
}
.btn.blue { background: var(--anh); box-shadow: 0 5px 0 #3355cc; }
.btn.ghost { background: #fff; color: var(--ink); box-shadow: inset 0 0 0 2px var(--line); }
.btn:hover, button[type=submit]:hover { transform: translateY(-2px); }
.btn:active, button[type=submit]:active { transform: translateY(2px); box-shadow: 0 2px 0 #d96a1f; }
.error, .banner {
  font-size: 1.15rem; font-weight: 800; padding: 1rem 1.15rem; border-radius: 1.2rem; margin: 1rem 0;
}
.error, .bad { color: var(--bad); background: #ffe5e5; }
.ok { color: var(--ok); background: #dcf7ec; }
.sheet {
  background: var(--sheet); border-radius: calc(var(--radius) + .3rem);
  padding: 1.4rem 1.4rem 1.6rem; box-shadow: 0 14px 30px rgba(31,42,68,.08);
  border: 2px solid var(--line);
}

/* lesson page */
.progress {
  display: flex; gap: .3rem; flex-wrap: wrap; margin: 0 0 .9rem; align-items: center;
  font-weight: 800; color: var(--muted); font-variant-numeric: tabular-nums;
}
.progress .tick { width: .65rem; height: .65rem; border-radius: 999px; background: #dbe4f5; }
.progress .tick.on { background: var(--toan); transform: scale(1.3); }
.star-count {
  margin-left: auto; color: #b07d00; background: #fff3c9;
  border-radius: 999px; padding: .15rem .7rem; font-size: 1.05rem;
}
.star-count.big { margin-left: 0; font-size: 1.3rem; padding: .35rem 1rem; }
.prompt {
  font-family: "Baloo 2", sans-serif; font-size: clamp(1.5rem, 4.5vw, 2.1rem);
  font-weight: 700; margin: .35rem 0 .2rem; text-wrap: pretty; line-height: 1.3;
}
.scene {
  display: grid; place-items: center; min-height: 9.5rem; margin: .9rem 0 1rem;
  padding: 1rem; background: var(--paper); border-radius: 1.3rem; border: 2px dashed var(--line);
}
.scene svg { width: 100%; max-width: 30rem; height: auto; display: block; }
.scene .big-emoji { font-size: clamp(5rem, 18vw, 7.5rem); line-height: 1; }
.count-hint { text-align: center; color: var(--muted); font-weight: 800; margin: .4rem 0 0; }
.speak {
  border: 0; cursor: pointer; background: var(--anh-soft); color: var(--anh);
  font: inherit; font-weight: 800; border-radius: 999px; padding: .45rem 1rem;
  display: inline-flex; align-items: center; gap: .4rem; margin: .2rem 0 .4rem;
}
.speak:hover { background: var(--anh); color: #fff; }
.choices { display: grid; grid-template-columns: 1fr 1fr; gap: .9rem; margin-top: 1.1rem; }
button.choice {
  min-height: 4.8rem; font-size: clamp(1.35rem, 4vw, 1.7rem); width: 100%;
  font-weight: 700; border: 2px solid var(--line); border-radius: 1.25rem;
  background: #fff; color: var(--ink); cursor: pointer;
  box-shadow: 0 6px 0 rgba(31,42,68,.08);
  transition: transform .12s ease, box-shadow .12s ease, background .12s ease;
}
.choices form:nth-child(1) button { background: var(--toan-soft); border-color: #ffd6b3; }
.choices form:nth-child(2) button { background: var(--viet-soft); border-color: #bfeed6; }
.choices form:nth-child(3) button { background: var(--anh-soft); border-color: #c9d6ff; }
.choices form:nth-child(4) button { background: #ffeef5; border-color: #ffd0e0; }
button.choice:hover { transform: translateY(-2px); }
button.choice:active { transform: translateY(2px); box-shadow: 0 2px 0 rgba(31,42,68,.08); }
button.choice.right { background: var(--ok); border-color: var(--ok); color: #fff; animation: pop .35s ease; }
button.choice.wrong { animation: shake .35s ease; background: #ffe5e5; border-color: var(--bad); color: var(--bad); }
.lessons { display: flex; flex-direction: column; gap: .55rem; }
a.item {
  display: flex; justify-content: space-between; align-items: center; gap: 1rem;
  text-decoration: none; color: inherit; background: var(--sheet);
  border: 2px solid var(--line); border-radius: 1.15rem; padding: .95rem 1.1rem;
  min-height: 3.6rem; font-weight: 800; box-shadow: 0 4px 10px rgba(31,42,68,.05);
}
a.item:hover { border-color: var(--toan); }
a.item span.n { color: var(--muted); font-variant-numeric: tabular-nums; }
a.item.done { opacity: .72; }
a.item.done:hover { opacity: 1; }
.done-mark {
  color: var(--ok); font-weight: 800; font-size: 1.2rem;
  background: #dcf7ec; border-radius: 999px; padding: .1rem .6rem;
}
.progress-line { color: var(--muted); font-weight: 700; margin: .2rem 0 1.2rem; }
.progress-line b { color: var(--ink); }
.report-rows { display: flex; flex-direction: column; gap: .7rem; }
a.report-row {
  display: grid; grid-template-columns: 6.5rem 1fr auto; gap: .9rem; align-items: center;
  text-decoration: none; color: inherit; background: var(--sheet);
  border: 2px solid var(--line); border-radius: 1.1rem; padding: .85rem 1rem;
}
.rlabel { font-family: "Baloo 2", sans-serif; font-weight: 700; font-size: 1.1rem; }
.rbar { height: .9rem; border-radius: 999px; background: #e8eefb; overflow: hidden; }
.rfill { display: block; height: 100%; border-radius: 999px; }
.rfill.toan { background: var(--toan); }
.rfill.viet { background: var(--viet); }
.rfill.anh { background: var(--anh); }
.rnum { font-weight: 800; color: var(--muted); font-variant-numeric: tabular-nums; }
.day-list { list-style: none; margin: 0; padding: 0; display: flex; flex-direction: column; gap: .45rem; }
.day-list li {
  display: flex; justify-content: space-between; gap: 1rem;
  background: var(--sheet); border: 2px solid var(--line); border-radius: .9rem;
  padding: .6rem .95rem; font-weight: 700; color: var(--muted);
}
.day-list li span:first-child { color: var(--ink); font-variant-numeric: tabular-nums; }
.n.warn { color: var(--bad); }
.hint-line { color: var(--muted); font-weight: 700; }
.cta-row { margin: 0 0 1.1rem; }
.done-actions { display: flex; gap: .7rem; flex-wrap: wrap; }
.chip {
  display: inline-flex; align-items: center; justify-content: center;
  padding: .35rem .8rem; border-radius: 999px; background: #fff;
  border: 2px solid var(--line); font-weight: 700; font-size: 1rem;
}
a.world:focus-visible, a.face:focus-visible, button:focus-visible, .btn:focus-visible,
input:focus-visible, select:focus-visible, a.item:focus-visible {
  outline: 3px solid var(--anh); outline-offset: 3px;
}
@keyframes pop { 0% { transform: scale(1); } 45% { transform: scale(1.12); } 100% { transform: scale(1); } }
.confetti {
  position: fixed; top: -1.2rem; width: .8rem; height: 1.1rem; border-radius: 3px;
  z-index: 60; pointer-events: none;
  animation: fall 1.6s ease-in forwards;
}
@keyframes fall {
  to { transform: translateY(105vh) rotate(540deg); opacity: .85; }
}
@keyframes shake {
  0%, 100% { transform: translateX(0); }
  25% { transform: translateX(-6px); } 75% { transform: translateX(6px); }
}
.letter-card {
  display: flex; align-items: center; justify-content: center; gap: 2.2rem;
  padding: 1rem 1.4rem; background: #fff; border-radius: 1.6rem;
  border: 3px dashed var(--line); max-width: 24rem;
}
.letter {
  font-family: "Baloo 2", sans-serif; font-weight: 800;
  font-size: clamp(4.5rem, 18vw, 7rem); line-height: 1.1; cursor: pointer;
  border-radius: 1.2rem; padding: .2rem .9rem;
  transition: transform .12s ease, background .12s ease;
}
.letter:hover { transform: translateY(-4px) scale(1.04); }
.letter.up { color: var(--toan); background: var(--toan-soft); }
.letter.lo { color: var(--viet); background: var(--viet-soft); }
.letter.rh { color: var(--anh); background: var(--anh-soft); }
.letter-card.rhyme { flex-direction: column; gap: .4rem; }
.letter-word {
  font-family: "Baloo 2", sans-serif; font-weight: 700;
  font-size: clamp(1.9rem, 7vw, 2.8rem); color: var(--ink); cursor: pointer;
  background: #fff6e3; border-radius: 1rem; padding: .25rem 1.1rem;
}
.letter-word.en { font-family: "Baloo 2", sans-serif; color: var(--anh); background: var(--anh-soft); }
.word-card {
  display: flex; align-items: center; gap: 2rem; padding: 1rem 1.5rem;
  background: #fff; border-radius: 1.6rem; border: 3px dashed #c9d6ff;
}
.letter.up { color: var(--anh); background: var(--anh-soft); }
.letter-card .letter.up { color: var(--toan); background: var(--toan-soft); }
.word-side { display: flex; flex-direction: column; align-items: center; gap: .3rem; }
.word-emoji { font-size: clamp(3.4rem, 13vw, 5rem); line-height: 1.2; cursor: pointer; }
@media (max-width: 640px) {
  .choices { grid-template-columns: 1fr; }
  a.world { grid-template-columns: minmax(3.6rem, 4.8rem) minmax(0, 1fr); gap: .75rem; padding: .9rem 1rem; }
  a.world .go { display: none; }
  .world .art { height: 4.2rem; }
}
@media (prefers-reduced-motion: reduce) {
  *, *::before, *::after { transition-duration: 0.01ms !important; animation-duration: 0.01ms !important; }
}
"#;

const SCRIPT: &str = r##"
(function () {
  "use strict";
  var VOICE = null;
  function pickVoice() {
    var vs = window.speechSynthesis ? speechSynthesis.getVoices() : [];
    for (var i = 0; i < vs.length; i++) {
      if (/^vi(-|_)/i.test(vs[i].lang)) { VOICE = vs[i]; return; }
    }
    VOICE = null;
  }
  if (window.speechSynthesis) {
    pickVoice();
    speechSynthesis.onvoiceschanged = pickVoice;
  }
  function say(text) {
    if (!window.speechSynthesis || !text) return;
    try {
      speechSynthesis.cancel();
      var u = new SpeechSynthesisUtterance(text);
      if (VOICE) u.voice = VOICE;
      u.lang = VOICE ? VOICE.lang : "vi-VN";
      u.rate = 0.92;
      u.pitch = 1.15;
      speechSynthesis.speak(u);
    } catch (e) { /* speech is a bonus */ }
  }
  window.__say = say;

  var EN_VOICE = null;
  function pickEnVoice() {
    var vs = window.speechSynthesis ? speechSynthesis.getVoices() : [];
    for (var i = 0; i < vs.length; i++) {
      if (/^en(-|_)/i.test(vs[i].lang)) { EN_VOICE = vs[i]; return; }
    }
    EN_VOICE = null;
  }
  if (window.speechSynthesis) {
    pickEnVoice();
    speechSynthesis.onvoiceschanged = function () { pickVoice(); pickEnVoice(); };
  }
  function sayEn(text) {
    if (!window.speechSynthesis || !text) return;
    try {
      speechSynthesis.cancel();
      var u = new SpeechSynthesisUtterance(text);
      if (EN_VOICE) u.voice = EN_VOICE;
      u.lang = EN_VOICE ? EN_VOICE.lang : "en-US";
      u.rate = 0.85;
      u.pitch = 1.1;
      speechSynthesis.speak(u);
    } catch (e) { /* speech is a bonus */ }
  }
  window.__sayEn = sayEn;

  function bindSayTaps(root) {
    var box = root || document;
    var nodes = box.querySelectorAll ? box.querySelectorAll("[data-say]") : [];
    Array.prototype.forEach.call(nodes, function (el) {
      el.classList.add("tap-say");
      el.addEventListener("click", function () {
        say(el.getAttribute("data-say"));
      });
    });
    var enNodes = box.querySelectorAll ? box.querySelectorAll("[data-say-en]") : [];
    Array.prototype.forEach.call(enNodes, function (el) {
      el.classList.add("tap-say");
      el.addEventListener("click", function () {
        sayEn(el.getAttribute("data-say-en"));
      });
    });
  }
  window.__bindSayTaps = bindSayTaps;

  function countables(root) {
    return Array.prototype.slice.call((root || document).querySelectorAll(".countable"));
  }
  function bindCountTap(root) {
    var box = root || document;
    var targets = countables(box);
    if (!targets.length) return;
    var hint = box.querySelector ? box.querySelector(".count-hint") : null;
    var n = 0;
    targets.forEach(function (el) {
      el.style.cursor = "pointer";
      el.addEventListener("click", function () {
        if (el.dataset.counted) return;
        el.dataset.counted = "1";
        n += 1;
        el.classList.add("counted");
        say(String(n));
        if (hint) hint.textContent = "Đã đếm: " + n;
      });
    });
  }
  window.__bindCountTap = bindCountTap;

  var speakBtn = document.querySelector("[data-say]");
  if (speakBtn) {
    speakBtn.addEventListener("click", function () {
      say(speakBtn.getAttribute("data-say"));
    });
  }

  bindSayTaps(document);

  // Auto-read the question once per page load so pre-readers can follow along.
  // Browsers block speech before the first user gesture; in that case the child
  // taps "Đọc đề" and every later page will auto-read.
  var autoText = speakBtn ? speakBtn.getAttribute("data-say") : null;
  if (autoText) {
    var unlocked = false;
    try { unlocked = speechSynthesis.speaking || speechSynthesis.pending; } catch (e) {}
    if (unlocked) {
      setTimeout(function () { say(autoText); }, 600);
    } else {
      var unlock = function () {
        document.removeEventListener("pointerdown", unlock);
        say(autoText);
      };
      document.addEventListener("pointerdown", unlock, { once: true });
    }
  }

  function burstConfetti() {
    var colors = ["#ff8a3d", "#2bb673", "#4f7cff", "#ffc531", "#ff5f9e"];
    for (var i = 0; i < 26; i++) {
      var s = document.createElement("span");
      s.className = "confetti";
      s.style.left = (45 + Math.random() * 10) + "%";
      s.style.background = colors[i % colors.length];
      s.style.transform = "rotate(" + (Math.random() * 360) + "deg)";
      s.style.animationDelay = (Math.random() * 0.25) + "s";
      document.body.append(s);
      (function (node) { setTimeout(function () { node.remove(); }, 1900); })(s);
    }
  }
  window.__confetti = burstConfetti;

  if (document.querySelector(".confetti-anchor")) {
    burstConfetti();
    say("Giỏi quá!");
  }

  bindCountTap(document);

  var wrongBtn = document.querySelector(".banner.bad");
  if (wrongBtn && window.__say) {
    say("Chưa đúng, thử lại nhé.");
  }
})();
"##;

fn dot(x: i64, y: i64, r: i64, fill: &str, counted: bool) -> String {
    format!(
        "<circle class='countable' data-n='1' cx='{x}' cy='{y}' r='{r}' fill='{fill}'{}",
        if counted { " opacity='.55'" } else { "" },
    )
}

fn group_dots(x0: i64, y0: i64, per_row: usize, count: usize, fill: &str) -> String {
    let mut s = String::new();
    for i in 0..count {
        let cx = x0 + (i % per_row) as i64 * 52;
        let cy = y0 + (i / per_row) as i64 * 52;
        s.push_str(&dot(cx, cy, 20, fill, false));
    }
    s
}

fn svg_wrap(inner: &str, w: i64, h: i64) -> String {
    format!(
        "<svg viewBox='0 0 {w} {h}' role='img' xmlns='http://www.w3.org/2000/svg'>{inner}</svg>"
    )
}

fn star(cx: i64, cy: i64, r: i64, counted: bool) -> String {
    // 5-point star path
    let mut pts = Vec::new();
    for i in 0..10 {
        let ang = -std::f64::consts::FRAC_PI_2 + (i as f64) * std::f64::consts::PI / 5.0;
        let rad = if i % 2 == 0 {
            r as f64
        } else {
            r as f64 * 0.45
        };
        let x = cx as f64 + rad * ang.cos();
        let y = cy as f64 + rad * ang.sin();
        pts.push(format!("{:.1},{:.1}", x, y));
    }
    let op = if counted { " opacity='.5'" } else { "" };
    format!(
        "<polygon class='countable' points='{}' fill='#ffc531' stroke='#e8a70f' stroke-width='2'{op}/>",
        pts.join(" ")
    )
}

fn apple(cx: i64, cy: i64, r: i64, counted: bool) -> String {
    let op = if counted { " opacity='.5'" } else { "" };
    format!(
        "<g class='countable'{op}><circle cx='{cx}' cy='{cy}' r='{r}' fill='#ff5a4e'/>\
<circle cx='{}' cy='{}' r='{}' fill='#ff8a80'/>\
<path d='M {cx} {} q 2 -10 10 -12' stroke='#3d8b47' stroke-width='4' fill='none' stroke-linecap='round'/></g>",
        cx - r / 3,
        cy - r / 3,
        r / 3,
        cy - r
    )
}

fn tomato(cx: i64, cy: i64, r: i64) -> String {
    format!(
        "<g class='countable'><circle cx='{cx}' cy='{cy}' r='{r}' fill='#ff5a4e'/>\
<circle cx='{}' cy='{}' r='{}' fill='#ff8a80'/>\
<path d='M {cx} {} l -7 -7 m 7 7 l 7 -7' stroke='#2e7d32' stroke-width='3' stroke-linecap='round'/></g>",
        cx - r / 3,
        cy - r / 3,
        r / 3,
        cy - r + 2
    )
}

fn bag_of_ten(x: i64, y: i64) -> String {
    let mut inner = String::new();
    // 3-4-3 pyramid of ten tomatoes inside the bag, centered on (x, y+50)
    let positions = [
        (-36, 20),
        (0, 20),
        (36, 20),
        (-18, 50),
        (18, 50),
        (0, 80),
        (-54, 20),
        (-18, 80),
        (18, 80),
        (54, 20),
    ];
    for (dx, dy) in positions {
        inner.push_str(&tomato(x + dx, y + dy, 16));
    }
    format!(
        "<g><path d='M {x} {y} c -52 6 -78 38 -78 82 c 0 44 32 68 78 68 c 46 0 78 -24 78 -68 c 0 -44 -26 -76 -78 -82 Z' fill='#bfe3ff' stroke='#7cc0f2' stroke-width='4'/>\
<path d='M {x} {y} l -16 -16 m 16 16 l 16 -16' stroke='#7cc0f2' stroke-width='5' stroke-linecap='round'/>{inner}</g>",
        x = x,
        y = y,
        inner = inner
    )
}

fn flat_shape(shape: FlatShape, x: i64, y: i64, s: i64, fill: &str, stroke: &str) -> String {
    let half = s / 2;
    match shape {
        FlatShape::Circle => format!(
            "<circle cx='{}' cy='{}' r='{}' fill='{fill}' stroke='{stroke}' stroke-width='5'/>",
            x + half,
            y + half,
            half
        ),
        FlatShape::Square => format!(
            "<rect x='{x}' y='{y}' width='{s}' height='{s}' rx='10' fill='{fill}' stroke='{stroke}' stroke-width='5'/>"
        ),
        FlatShape::Rectangle => format!(
            "<rect x='{x}' y='{}' width='{}' height='{}' rx='10' fill='{fill}' stroke='{stroke}' stroke-width='5'/>",
            y + s / 4,
            s + 60,
            s / 2
        ),
        FlatShape::Triangle => format!(
            "<polygon points='{x},{} {},{} {},{}' fill='{fill}' stroke='{stroke}' stroke-width='5' stroke-linejoin='round'/>",
            y + s,
            x + s,
            y + s,
            x + half,
            y
        ),
    }
}

fn solid_shape(shape: SolidShape, x: i64, y: i64, s: i64) -> String {
    let d = s / 3; // depth offset for the 3D look
    match shape {
        SolidShape::Cube => {
            let (x1, y1) = (x + d, y - d); // top face back edge
            let (x2, y2) = (x + s + d, y - d + s); // side face far bottom
            format!(
                "<g>\
<polygon points='{x},{y} {x1},{y1} {x2},{y2} {},{}' fill='#ffd28a' stroke='#d9930f' stroke-width='4'/>\
<polygon points='{x},{y} {x1},{y1} {},{} {x},{y2}' fill='#ffc531' stroke='#d9930f' stroke-width='4'/>\
<rect x='{x}' y='{y}' width='{s}' height='{s}' fill='#ffe08a' stroke='#d9930f' stroke-width='4'/></g>",
                x + s + d, y2,
                x + d, y + s,
            )
        }
        SolidShape::Cuboid => {
            let y14 = y + 14;
            let x_far = x + d + 84;
            let y_top = y + 14 - d;
            format!(
                "<g>\
<polygon points='{x},{y14} {x1},{ytop} {xfar},{ytop2} {x84b},{y14b}' fill='#bfe3ff' stroke='#4f7cff' stroke-width='4'/>\
<polygon points='{x84},{y14} {x1},{ytop} {x1},{ybot} {x84},{ybot2}' fill='#9cc4ff' stroke='#4f7cff' stroke-width='4'/>\
<rect x='{x}' y='{y14}' width='84' height='{h}' fill='#dce9ff' stroke='#4f7cff' stroke-width='4'/></g>",
                x1 = x + d,
                ytop = y,
                ytop2 = y_top,
                xfar = x_far,
                x84 = x + 84,
                x84b = x + d + 84,
                y14 = y14,
                y14b = y + 14 - d,
                ybot = y_top + s - 14,
                ybot2 = y + 14 + s - 14,
                h = s - 14,
            )
        }
        SolidShape::Sphere => format!(
            "<g><circle cx='{}' cy='{}' r='{}' fill='#ff9db8' stroke='#e0517a' stroke-width='4'/>\
<ellipse cx='{}' cy='{}' rx='{}' ry='{}' fill='none' stroke='#e0517a' stroke-width='3' opacity='.6'/></g>",
            x + s, y + s, s,
            x + s, y + s, s, s / 3
        ),
    }
}

fn ruler_svg() -> String {
    let mut ticks = String::new();
    for i in 0..11 {
        let x = 60 + i * 36;
        let len = if i % 5 == 0 { 26 } else { 16 };
        ticks.push_str(&format!(
            "<line x1='{x}' y1='150' x2='{x}' y2='{}' stroke='#4f7cff' stroke-width='3'/>",
            150 + len
        ));
        if i % 5 == 0 && i > 0 {
            ticks.push_str(&format!(
                "<text x='{}' y='195' text-anchor='middle' font-size='17' fill='#4f7cff' font-family='Nunito'>{}</text>",
                x, i / 5
            ));
        }
    }
    svg_wrap(
        &format!(
            "<g class='countable'><rect x='250' y='26' width='190' height='22' rx='8' fill='#ff8a3d'/>\
<polygon points='440,26 470,37 440,48' fill='#ff8a3d'/><rect x='258' y='32' width='150' height='5' rx='2' fill='#ffc531'/></g>\
<rect x='24' y='140' width='392' height='64' rx='12' fill='#dce9ff' stroke='#4f7cff' stroke-width='4'/>{ticks}"
        ),
        440,
        210,
    )
}

fn clock_svg(hour: u8, minute: u8) -> String {
    let cx = 120i64;
    let cy = 120i64;
    let hour_angle = ((hour % 12) as f64 * 30.0 + minute as f64 * 0.5) - 90.0;
    let minute_angle = minute as f64 * 6.0 - 90.0;
    let hx = cx + (58.0 * hour_angle.to_radians().cos()) as i64;
    let hy = cy + (58.0 * hour_angle.to_radians().sin()) as i64;
    let mx = cx + (84.0 * minute_angle.to_radians().cos()) as i64;
    let my = cy + (84.0 * minute_angle.to_radians().sin()) as i64;
    let mut ticks = String::new();
    for i in 0..12 {
        let a = i as f64 * 30.0f64.to_radians();
        let x1 = cx + (92.0 * a.cos()) as i64;
        let y1 = cy + (92.0 * a.sin()) as i64;
        let x2 = cx + (102.0 * a.cos()) as i64;
        let y2 = cy + (102.0 * a.sin()) as i64;
        ticks.push_str(&format!(
            "<line x1='{x1}' y1='{y1}' x2='{x2}' y2='{y2}' stroke='#4f7cff' stroke-width='4'/>"
        ));
        if i % 3 == 0 {
            let nx = cx + (72.0 * a.cos()) as i64;
            let ny = cy + (72.0 * a.sin()) as i64;
            let label = (i + 3) % 12;
            ticks.push_str(&format!(
                "<text x='{nx}' y='{}' text-anchor='middle' font-size='22' font-weight='800' fill='#1f2a44' font-family='Nunito'>{}</text>",
                ny + 8,
                if label == 0 { 12 } else { label }
            ));
        }
    }
    svg_wrap(
        &format!(
            "<circle cx='{cx}' cy='{cy}' r='110' fill='#fff' stroke='#4f7cff' stroke-width='8'/>\
{ticks}\
<line x1='{cx}' y1='{cy}' x2='{hx}' y2='{hy}' stroke='#ff8a3d' stroke-width='10' stroke-linecap='round'/>\
<line x1='{cx}' y1='{cy}' x2='{mx}' y2='{my}' stroke='#1f2a44' stroke-width='6' stroke-linecap='round'/>\
<circle cx='{cx}' cy='{cy}' r='7' fill='#ff8a3d'/>"
        ),
        240,
        240,
    )
}

fn chart100_svg(hi: u32) -> String {
    let mut cells = String::new();
    let start = if hi <= 10 { 1 } else { hi - 9 };
    for n in start..=start + 19 {
        let i = (n - start) as i64;
        let x = 20 + (i % 10) * 44;
        let y = 20 + (i / 10) * 44;
        let (fill, txt, weight) = if n == hi {
            ("#ffc531", n.to_string(), "800")
        } else if n == hi + 1 {
            ("#fff", "?".to_string(), "800")
        } else {
            ("#fff", n.to_string(), "600")
        };
        cells.push_str(&format!(
            "<rect x='{x}' y='{y}' width='40' height='40' rx='8' fill='{fill}' stroke='#c9d6ff'/>\
<text x='{}' y='{}' text-anchor='middle' font-size='19' font-weight='{weight}' fill='#1f2a44' font-family='Nunito'>{txt}</text>",
            x + 20,
            y + 27
        ));
    }
    svg_wrap(&cells, 480, 152)
}

fn week_svg() -> String {
    let days = ["Hai", "Ba", "Tư", "Năm", "Sáu", "Bảy", "CN"];
    let mut chips = String::new();
    for (i, d) in days.iter().enumerate() {
        let x = 16 + i as i64 * 62;
        let (fill, stroke) = if i == 0 {
            ("#ffc531", "#e8a70f")
        } else {
            ("#fff", "#c9d6ff")
        };
        chips.push_str(&format!(
            "<rect x='{x}' y='30' width='54' height='46' rx='14' fill='{fill}' stroke='{stroke}' stroke-width='2'/>\
<text x='{}' y='{}' text-anchor='middle' font-size='19' font-weight='800' fill='#1f2a44' font-family='Nunito'>{d}</text>",
            x + 27,
            59
        ));
    }
    svg_wrap(&chips, 460, 106)
}

fn shape_row_svg() -> String {
    let shapes = [
        (FlatShape::Circle, "#ff5a4e", "#e05143"),
        (FlatShape::Square, "#4f7cff", "#3355cc"),
        (FlatShape::Triangle, "#2bb673", "#1f9660"),
        (FlatShape::Rectangle, "#ffc531", "#e8a70f"),
    ];
    let mut out = String::new();
    for (i, (shape, fill, stroke)) in shapes.iter().enumerate() {
        out.push_str(&flat_shape(
            *shape,
            20 + i as i64 * 112,
            24,
            80,
            fill,
            stroke,
        ));
    }
    svg_wrap(&out, 480, 130)
}

fn scene_svg(picture: Picture) -> String {
    match picture {
        Picture::None => String::new(),
        Picture::Stars(n) => {
            let mut g = String::new();
            for i in 0..n as i64 {
                g.push_str(&star(30 + i * 72, 55, 26, false));
            }
            svg_wrap(&g, 24 + n as i64 * 72, 110)
        }
        Picture::Apples(n) => {
            if n == 0 {
                return "<div class='pic empty'>Không có quả nào — số 0!</div>".into();
            }
            let mut g = String::new();
            for i in 0..n as i64 {
                let row = i / 4;
                let col = i % 4;
                g.push_str(&apple(40 + col * 76, 50 + row * 76, 30, false));
            }
            svg_wrap(
                &g,
                24 + (n as i64).min(4) * 76,
                100 + (n as i64 - 1) / 4 * 76,
            )
        }
        Picture::Dots(n) => svg_wrap(&group_dots(40, 40, 5, n as usize, "#4f7cff"), 300, 150),
        Picture::Blocks(n) => {
            let mut g = String::new();
            for i in 0..n as i64 {
                g.push_str(&format!(
                    "<rect class='countable' x='{}' y='40' width='42' height='42' rx='9' fill='#ff8a3d'/>",
                    30 + i * 54
                ));
            }
            svg_wrap(&g, 60 + n as i64 * 54, 120)
        }
        Picture::Compare(a, b) => {
            let g = format!(
                "{}<text x='240' y='92' text-anchor='middle' font-size='40' font-weight='800' fill='#6b7a99' font-family='Nunito'>?</text>{}",
                group_dots(30, 40, 3, a as usize, "#ff8a3d"),
                group_dots(300, 40, 3, b as usize, "#2bb673")
            );
            svg_wrap(&g, 540, 190)
        }
        Picture::Join(a, b) => {
            let g = format!(
                "{}<text x='240' y='92' text-anchor='middle' font-size='44' font-weight='800' fill='#6b7a99' font-family='Nunito'>+</text>{}",
                group_dots(30, 40, 3, a as usize, "#ff8a3d"),
                group_dots(300, 40, 3, b as usize, "#2bb673")
            );
            svg_wrap(&g, 540, 190)
        }
        Picture::Tomatoes(chuc, don_vi) => {
            let mut g = String::new();
            for i in 0..chuc as i64 {
                g.push_str(&bag_of_ten(110 + i * 200, 40));
            }
            let loose_x0 = 110 + chuc as i64 * 200;
            for i in 0..don_vi as i64 {
                g.push_str(&tomato(loose_x0 + i * 54, 150, 22));
            }
            let w = (loose_x0 + don_vi as i64 * 54 + 40).max(240);
            svg_wrap(&g, w, 210)
        }
        Picture::Chart100(hi) => chart100_svg(hi),
        Picture::Clock(h, m) => clock_svg(h, m),
        Picture::Week => week_svg(),
        Picture::FlatShape(shape) => {
            let (fill, stroke) = match shape {
                FlatShape::Circle => ("#ff5a4e", "#e05143"),
                FlatShape::Square => ("#4f7cff", "#3355cc"),
                FlatShape::Triangle => ("#2bb673", "#1f9660"),
                FlatShape::Rectangle => ("#ffc531", "#e8a70f"),
            };
            svg_wrap(&flat_shape(shape, 40, 30, 130, fill, stroke), 210, 190)
        }
        Picture::FlatShapes => shape_row_svg(),
        Picture::SolidShape(shape) => svg_wrap(&solid_shape(shape, 70, 80, 130), 300, 220),
        Picture::Ruler => ruler_svg(),
        Picture::Emoji(e) => format!("<div class='big-emoji'>{}</div>", escape(e)),
        Picture::LetterCard(up, lo) => {
            format!(
                "<div class='letter-card'>\
<span class='letter up' data-say='{up}'>{up}</span>\
<span class='letter lo' data-say='{lo}'>{lo}</span></div>",
                up = escape(up),
                lo = escape(lo),
            )
        }
        Picture::RhymeCard(rhyme, word) => {
            format!(
                "<div class='letter-card rhyme'>\
<span class='letter rh' data-say='{rh}'>{rh}</span>\
<span class='letter-word' data-say='{word}'>{word}</span></div>",
                rh = escape(rhyme),
                word = escape(word),
            )
        }
        Picture::WordCard(letter, word, emoji) => {
            format!(
                "<div class='word-card'>\
<span class='letter up' data-say-en='{letter}'>{letter}</span>\
<div class='word-side'>\
<span class='word-emoji' data-say-en='{word}'>{emoji}</span>\
<span class='letter-word en' data-say-en='{word}'>{word}</span></div></div>",
                letter = escape(letter),
                word = escape(word),
                emoji = escape(emoji),
            )
        }
    }
}

fn scene_block(picture: Picture, prompt: &str) -> String {
    let _ = prompt;
    match picture {
        Picture::None => String::new(),
        Picture::Stars(_)
        | Picture::Apples(_)
        | Picture::Dots(_)
        | Picture::Blocks(_)
        | Picture::Compare(_, _)
        | Picture::Join(_, _)
        | Picture::Tomatoes(_, _) => {
            let hint = "<p class=count-hint>Chạm vào từng hình để đếm nhé!</p>";
            format!("<div class=scene>{}</div>{hint}", scene_svg(picture))
        }
        Picture::LetterCard(_, _) => {
            let hint = "<p class=count-hint>Chạm vào từng chữ để nghe đọc nhé!</p>";
            format!("<div class=scene>{}</div>{hint}", scene_svg(picture))
        }
        Picture::RhymeCard(_, _) => {
            let hint = "<p class=count-hint>Chạm vào vần hoặc từ để nghe đọc nhé!</p>";
            format!("<div class=scene>{}</div>{hint}", scene_svg(picture))
        }
        Picture::WordCard(_, _, _) => {
            let hint = "<p class=count-hint>Chạm vào chữ, tranh hoặc từ để nghe đọc nhé!</p>";
            format!("<div class=scene>{}</div>{hint}", scene_svg(picture))
        }
        Picture::Emoji(_) => {
            let hint = "<p class=count-hint>Hãy nói câu trả lời của bé nhé!</p>";
            format!("<div class=scene>{}</div>{hint}", scene_svg(picture))
        }
        _ => format!("<div class=scene>{}</div>", scene_svg(picture)),
    }
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

pub fn home(profile: &Profile, stars: usize) -> String {
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
            "<div class=nav><a href=/profiles>← Đổi hồ sơ</a><a href=/profiles/{pid}/bao-cao>📊 Báo cáo của bố mẹ</a></div>\
<div class=hello><span class=bubble tiny>{emoji}</span>\
<div><h1>Xin chào, {name}!</h1>\
<p class=sub>Chọn một phòng học. Bé làm từng bài, không cần đọc chữ nhỏ.</p></div>\
<span class=star-count big aria-label='tổng sao'>⭐ {stars} sao</span></div>\
<div class=worlds>{worlds}</div>",
            pid = profile.id,
            name = escape(&profile.name),
            emoji = avatar_emoji(&profile.avatar_key),
            stars = stars,
        ),
    )
}

pub fn subject_page(
    profile: &Profile,
    subject: Subject,
    lessons: &[&Lesson],
    done: &std::collections::HashSet<u32>,
    start_id: Option<u32>,
) -> String {
    let mut list = String::new();
    let mut last_unit = "";
    for (index, lesson) in lessons.iter().enumerate() {
        if lesson.unit != last_unit {
            list.push_str(&format!("<h2 class=unit>{}</h2>", escape(lesson.unit)));
            last_unit = lesson.unit;
        }
        let mark = if done.contains(&lesson.id) {
            "<span class='n done-mark' aria-label='đã học xong'>✓</span>".to_string()
        } else {
            format!("<span class=n>Bài {}</span>", index + 1)
        };
        list.push_str(&format!(
            "<a class='item{}' href=/profiles/{pid}/bai/{lid}><span>{title}</span>{mark}</a>",
            if done.contains(&lesson.id) {
                " done"
            } else {
                ""
            },
            pid = profile.id,
            lid = lesson.id,
            title = escape(lesson.title),
            mark = mark,
        ));
    }
    let done_count = lessons.iter().filter(|l| done.contains(&l.id)).count();
    let start = match start_id {
        Some(lesson_id) => format!(
            "<p class=cta-row><a class=btn href=/profiles/{}/bai/{}>Học tiếp ✨</a></p>\
<p class=progress-line>Đã học: <b>{done_count}</b>/{count} bài</p>",
            profile.id,
            lesson_id,
            count = lessons.len(),
        ),
        None => format!(
            "<p class=cta-row><a class='btn ghost' href=/profiles/{}/mon/{}>Hoàn thành! Ôn lại từ bài 1</a></p>\
<p class=progress-line>🎉 Đã học đủ <b>{done_count}</b>/{count} bài</p>",
            profile.id,
            subject.slug(),
            done_count = done_count,
            count = lessons.len(),
        ),
    };
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

/// Parent-facing report: per-subject progress bars and recent daily activity.
pub fn report_page(
    profile: &Profile,
    per_subject: &[(Subject, usize, usize)],
    days: &[DayStat],
    hard_lessons: &[&Lesson],
) -> String {
    let mut rows = String::new();
    for (subject, done, total) in per_subject {
        let pct = if *total == 0 { 0 } else { done * 100 / total };
        let class = match subject {
            Subject::Toan => "toan",
            Subject::TiengViet => "viet",
            Subject::TiengAnh => "anh",
        };
        rows.push_str(&format!(
            "<a class='report-row' href=/profiles/{pid}/mon/{slug}>\
<span class=rlabel>{title}</span>\
<span class=rbar><span class='rfill {class}' style='width:{pct}%'></span></span>\
<span class=rnum>{done}/{total}</span></a>",
            pid = profile.id,
            slug = subject.slug(),
            title = subject.title(),
        ));
    }
    let mut day_rows = String::new();
    for stat in days {
        day_rows.push_str(&format!(
            "<li><span>{day}</span><span>✓ {lessons_done} bài · {answers} lượt trả lời</span></li>",
            day = escape(&stat.day),
            lessons_done = stat.lessons_done,
            answers = stat.answers,
        ));
    }
    let day_block = if day_rows.is_empty() {
        "<p class=sub>Bé chưa học buổi nào. Chọn một môn để bắt đầu nhé!</p>".to_string()
    } else {
        format!("<ul class=day-list>{day_rows}</ul>")
    };
    let mut hard = String::new();
    for lesson in hard_lessons {
        hard.push_str(&format!(
            "<a class=item href=/profiles/{pid}/bai/{lid}><span>{title}</span><span class='n warn'>hay sai ✗</span></a>",
            pid = profile.id,
            lid = lesson.id,
            title = escape(lesson.title),
        ));
    }
    let hard_block = if hard.is_empty() {
        "<p class=sub>Chưa có bài nào bé hay sai. Rất tốt!</p>".to_string()
    } else {
        format!("<div class=lessons>{hard}</div><p class=hint-line>Chạm vào bài để luyện lại cùng bé.</p>")
    };
    page(
        "Báo cáo",
        &format!(
            "<div class=nav><a href=/profiles/{pid}>← Hồ sơ của {name}</a></div>\
<h1>Báo cáo học tập</h1><p class=sub>Tiến độ và hoạt động gần đây của bé {name}.</p>\
<h2 class=unit>Tiến độ từng môn</h2><div class=report-rows>{rows}</div>\
<h2 class=unit>Hoạt động gần đây</h2>{day_block}\
<h2 class=unit>Bài bé hay sai</h2>{hard_block}",
            pid = profile.id,
            name = escape(&profile.name),
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
    stars: usize,
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
                "<div class='banner ok confetti-anchor'>⭐ +1 sao! Giỏi quá, đúng rồi.</div><div class=done-actions>{next}\
<a class='btn ghost' href=/profiles/{pid}>Về chọn môn</a></div>",
                pid = profile.id,
                next = next
            )
        }
        Some(Flash::Wrong) => {
            "<div class='banner bad'>Chưa đúng. Thử lại nhé, bé ơi!</div>".to_string()
        }
        None => String::new(),
    };
    let mut dots = String::new();
    for i in 0..total {
        let on = if i == index { " on" } else { "" };
        dots.push_str(&format!("<span class='tick{on}'></span>"));
    }
    let scene = scene_block(lesson.picture, lesson.prompt);
    page(
        lesson.title,
        &format!(
            "<div class=nav><a href=/profiles/{pid}/mon/{slug}>{subject}</a></div>\
<div class=progress aria-label='Bài {n} trên {total}'>{dots}<span>Bài {n}/{total}</span>\
<span class=star-count aria-label='sao thưởng'>⭐ {stars}</span></div>\
<div class=sheet><p class=sub>{unit} · {title}</p>\
<button type=button class=speak data-say='{say}'>🔊 Đọc đề</button>\
{scene}<p class=prompt>{prompt}</p>{banner}<div class=choices>{choices}</div></div>",
            pid = profile.id,
            slug = lesson.subject.slug(),
            subject = lesson.subject.title(),
            n = index + 1,
            total = total,
            dots = dots,
            unit = escape(lesson.unit),
            title = escape(lesson.title),
            say = escape(lesson.prompt),
            scene = scene,
            prompt = escape(lesson.prompt),
        ),
    )
}

#[derive(Debug, Clone, Copy)]
pub enum Flash {
    Correct { next_id: Option<u32> },
    Wrong,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn scenes_render_for_each_kind() {
        let samples = [
            Picture::Stars(3),
            Picture::Apples(0),
            Picture::Apples(8),
            Picture::Dots(6),
            Picture::Blocks(5),
            Picture::Compare(5, 3),
            Picture::Join(4, 4),
            Picture::Tomatoes(2, 3),
            Picture::Chart100(29),
            Picture::Clock(3, 0),
            Picture::Week,
            Picture::FlatShape(FlatShape::Triangle),
            Picture::FlatShapes,
            Picture::SolidShape(SolidShape::Cube),
            Picture::Ruler,
            Picture::Emoji("🐱"),
        ];
        for pic in samples {
            let html = scene_svg(pic);
            assert!(
                html.starts_with("<svg") || html.contains("big-emoji") || html.contains("empty"),
                "{html:?}"
            );
        }
        assert!(scene_svg(Picture::None).is_empty());
    }

    #[test]
    fn countable_scene_has_hint_and_script_marks() {
        let html = scene_block(Picture::Stars(3), "Có bao nhiêu ngôi sao?");
        assert!(html.contains("countable"));
        assert!(html.contains("count-hint"));
        let plain = scene_block(Picture::Clock(3, 0), "Mấy giờ?");
        assert!(!plain.contains("count-hint"));
    }

    #[test]
    fn lesson_page_has_speak_button_and_scene() {
        let profile = Profile {
            id: 1,
            user_id: 1,
            name: "An".into(),
            avatar_key: "robot".into(),
            sort_order: 0,
        };
        let lesson = crate::lessons::by_id(1).unwrap();
        let html = lesson_page(&profile, lesson, 0, 54, 0, None);
        assert!(html.contains("data-say"));
        assert!(html.contains("countable"));
        assert!(html.contains("Chạm vào từng hình"));
    }

    #[test]
    fn confetti_anchor_on_correct_banner() {
        let profile = Profile {
            id: 1,
            user_id: 1,
            name: "An".into(),
            avatar_key: "robot".into(),
            sort_order: 0,
        };
        let lesson = crate::lessons::by_id(1).unwrap();
        let html = lesson_page(
            &profile,
            lesson,
            0,
            54,
            7,
            Some(Flash::Correct { next_id: Some(2) }),
        );
        assert!(html.contains("confetti-anchor"));
        assert!(html.contains("⭐ 7"));
    }
}
