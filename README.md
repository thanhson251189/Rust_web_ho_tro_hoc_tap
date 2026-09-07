# Web hỗ trợ học tập lớp 1

Private family site for grade-1 practice: Toán, Tiếng Việt, Tiếng Anh.

This repo follows [engineering-practices](https://github.com/thanhson251189/engineering-practices).
Read `AGENTS.md` before any edit.

## Scope now

- Grade 1 only: Toán, Tiếng Việt, Tiếng Anh (KNTT as the lesson outline).
- Google login for the parent.
- Netflix-style profile picker, maximum 2 child profiles per account.
- Progress stored per profile.
- Stack: Axum now; HTML templates and SQLite later.

## Out of scope

- Hosting textbook pages, PDFs, or scans from third-party sites.
- YouTube downloads (embed by video id later).
- Billing, plans, or school/class features.
- Extra subjects or grades until the first three work.

## Run

```text
cargo test
cargo run
```

Open http://127.0.0.1:3000 — the page is the greeting `Xin chào` until lessons exist.
