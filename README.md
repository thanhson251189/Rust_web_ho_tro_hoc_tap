# Web ho tro hoc tap lop 1

Private family site for grade-1 practice: Toan, Tieng Viet, Tieng Anh.

This repo follows [engineering-practices](https://github.com/thanhson251189/engineering-practices).
Read `AGENTS.md` before any edit.

## Scope now

- Grade 1 only: Toan, Tieng Viet, Tieng Anh (KNTT as the lesson outline).
- Google login for the parent (not wired yet).
- Netflix-style profile picker, maximum 2 child profiles per account.
- After opening a profile: hub of 3 grade-1 subjects. Toán follows the KNTT SGK tập 1–2 lesson outline (original tap-to-answer items, not textbook scans). Tiếng Việt / Tiếng Anh still have a short starter set.
- Profiles stored in `data/app.sqlite` when you `cargo run`.

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

Open http://127.0.0.1:3000 — choose or add a child profile, pick a subject, then tap an answer.
The SQLite file is local and gitignored.
