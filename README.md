# Web ho tro hoc tap lop 1

Private family site for grade-1 practice: Toan, Tieng Viet, Tieng Anh.

This repo follows [engineering-practices](https://github.com/thanhson251189/engineering-practices).
Read `AGENTS.md` before any edit.

## Scope now

- Grade 1 only: Toan, Tieng Viet, Tieng Anh (KNTT as the lesson outline).
- Google login for the parent (not wired yet).
- Netflix-style profile picker, maximum 2 child profiles per account.
- After opening a profile: hub of 3 grade-1 subjects. Practice items follow SGK outlines (Toán KNTT, Tiếng Việt KNTT, Tiếng Anh Global Success) — original tap-to-answer, not textbook scans.
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

## Let the family reach it (phone, tablet)

The server binds to localhost only, so by default only this machine can open it.
To use it from another device on the home Wi-Fi:

```text
HOTRO_ADDR=0.0.0.0:3000 cargo run
```

Then open `http://<LAN-IP-of-this-machine>:3000` on the phone or tablet.
`HOTRO_ADDR` also takes another port, e.g. `HOTRO_ADDR=0.0.0.0:8080`.

> **Warning:** the app has no login yet. Once bound to `0.0.0.0`, anyone on the
> same Wi-Fi can create profiles and read the study reports. That is a conscious
> trade-off for a home network, not an oversight. Keep it on localhost until
> Google login lands.
