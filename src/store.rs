use rusqlite::{params, Connection, OptionalExtension};
use std::path::Path;

pub const MAX_PROFILES_PER_USER: usize = 2;

/// Answers are stored in UTC, but a Vietnamese parent reads the report in local
/// time. Vietnam is UTC+7 all year (no DST), so one fixed offset is enough to
/// move a stored stamp onto the calendar day the parent actually saw.
const VN_OFFSET: &str = "+7 hours";

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct User {
    pub id: i64,
    pub google_sub: String,
    pub email: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Profile {
    pub id: i64,
    pub user_id: i64,
    pub name: String,
    pub avatar_key: String,
    pub sort_order: i64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum StoreError {
    ProfileLimit,
    UserNotFound,
    Db(String),
}

impl From<rusqlite::Error> for StoreError {
    fn from(err: rusqlite::Error) -> Self {
        StoreError::Db(err.to_string())
    }
}

pub fn open_memory() -> Result<Connection, StoreError> {
    let conn = Connection::open_in_memory()?;
    migrate(&conn)?;
    Ok(conn)
}

pub fn open_file(path: impl AsRef<Path>) -> Result<Connection, StoreError> {
    let path = path.as_ref();
    if let Some(dir) = path.parent() {
        if !dir.as_os_str().is_empty() {
            std::fs::create_dir_all(dir).map_err(|err| StoreError::Db(err.to_string()))?;
        }
    }
    let conn = Connection::open(path)?;
    migrate(&conn)?;
    Ok(conn)
}

pub fn migrate(conn: &Connection) -> Result<(), StoreError> {
    conn.execute_batch(
        "
        PRAGMA foreign_keys = ON;
        CREATE TABLE IF NOT EXISTS users (
            id INTEGER PRIMARY KEY,
            google_sub TEXT NOT NULL UNIQUE,
            email TEXT NOT NULL,
            created_at TEXT NOT NULL DEFAULT (datetime('now'))
        );
        CREATE TABLE IF NOT EXISTS profiles (
            id INTEGER PRIMARY KEY,
            user_id INTEGER NOT NULL REFERENCES users(id) ON DELETE CASCADE,
            name TEXT NOT NULL,
            avatar_key TEXT NOT NULL,
            sort_order INTEGER NOT NULL,
            created_at TEXT NOT NULL DEFAULT (datetime('now'))
        );
        CREATE TABLE IF NOT EXISTS progress (
            profile_id INTEGER NOT NULL REFERENCES profiles(id) ON DELETE CASCADE,
            lesson_id INTEGER NOT NULL,
            correct_count INTEGER NOT NULL DEFAULT 0,
            wrong_count INTEGER NOT NULL DEFAULT 0,
            done INTEGER NOT NULL DEFAULT 0,
            last_tried_at TEXT NOT NULL DEFAULT (datetime('now')),
            PRIMARY KEY (profile_id, lesson_id)
        );
        CREATE TABLE IF NOT EXISTS answer_events (
            id INTEGER PRIMARY KEY,
            profile_id INTEGER NOT NULL REFERENCES profiles(id) ON DELETE CASCADE,
            lesson_id INTEGER NOT NULL,
            correct INTEGER NOT NULL,
            at TEXT NOT NULL
        );
        ",
    )?;
    Ok(())
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Progress {
    pub profile_id: i64,
    pub lesson_id: u32,
    pub correct_count: i64,
    pub wrong_count: i64,
    pub done: bool,
}

/// Record one answer now (UTC). See [`record_answer_at`] for the timestamp seam.
///
/// `done` flips on the first correct answer and stays on, so a wrong retry never
/// un-finishes a lesson.
pub fn record_answer(
    conn: &Connection,
    profile_id: i64,
    lesson_id: u32,
    correct: bool,
) -> Result<Progress, StoreError> {
    record_answer_at(conn, profile_id, lesson_id, correct, &now_utc(conn)?)
}

/// Record one answer stamped `at`, a UTC timestamp ("YYYY-MM-DD HH:MM:SS").
/// Tests spread answers over several days through this entry point.
pub fn record_answer_at(
    conn: &Connection,
    profile_id: i64,
    lesson_id: u32,
    correct: bool,
    at: &str,
) -> Result<Progress, StoreError> {
    // Both writes land or neither does: `progress` alone would count a star the
    // report cannot explain. The caller holds the connection behind a mutex, so
    // `unchecked_transaction` (no `&mut` needed) is safe here.
    let tx = conn.unchecked_transaction()?;
    tx.execute(
        "INSERT INTO progress (profile_id, lesson_id, correct_count, wrong_count, done, last_tried_at)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6)
         ON CONFLICT(profile_id, lesson_id) DO UPDATE SET
            correct_count = correct_count + ?3,
            wrong_count = wrong_count + ?4,
            done = done | ?5,
            last_tried_at = ?6",
        params![
            profile_id,
            lesson_id,
            i64::from(correct),
            i64::from(!correct),
            i64::from(correct),
            at
        ],
    )?;
    // `progress` holds a single row per lesson, so its counters only ever
    // describe the last visit. Keep every attempt in its own row instead.
    tx.execute(
        "INSERT INTO answer_events (profile_id, lesson_id, correct, at) VALUES (?1, ?2, ?3, ?4)",
        params![profile_id, lesson_id, i64::from(correct), at],
    )?;
    let progress = tx.query_row(
        "SELECT profile_id, lesson_id, correct_count, wrong_count, done
         FROM progress WHERE profile_id = ?1 AND lesson_id = ?2",
        params![profile_id, lesson_id],
        |row| {
            Ok(Progress {
                profile_id: row.get(0)?,
                lesson_id: row.get(1)?,
                correct_count: row.get(2)?,
                wrong_count: row.get(3)?,
                done: row.get::<_, i64>(4)? != 0,
            })
        },
    )?;
    tx.commit()?;
    Ok(progress)
}

/// SQLite's own clock, so the schema defaults and the app agree on "now".
fn now_utc(conn: &Connection) -> Result<String, StoreError> {
    conn.query_row("SELECT datetime('now')", [], |row| row.get(0))
        .map_err(StoreError::from)
}

pub fn progress_for_profile(
    conn: &Connection,
    profile_id: i64,
) -> Result<Vec<Progress>, StoreError> {
    let mut stmt = conn.prepare(
        "SELECT profile_id, lesson_id, correct_count, wrong_count, done
         FROM progress WHERE profile_id = ?1",
    )?;
    let rows = stmt.query_map(params![profile_id], |row| {
        Ok(Progress {
            profile_id: row.get(0)?,
            lesson_id: row.get(1)?,
            correct_count: row.get(2)?,
            wrong_count: row.get(3)?,
            done: row.get::<_, i64>(4)? != 0,
        })
    })?;
    let mut out = Vec::new();
    for row in rows {
        out.push(row?);
    }
    Ok(out)
}

/// First unfinished lesson id of `subject`, following the bank's own order.
/// `None` means every lesson of the subject is done (or the subject is empty).
pub fn next_lesson_id(
    conn: &Connection,
    profile_id: i64,
    subject_lesson_ids: &[u32],
) -> Result<Option<u32>, StoreError> {
    let done: std::collections::HashSet<u32> = progress_for_profile(conn, profile_id)?
        .into_iter()
        .filter(|p| p.done)
        .map(|p| p.lesson_id)
        .collect();
    Ok(subject_lesson_ids
        .iter()
        .copied()
        .find(|id| !done.contains(id)))
}

/// Star total: one star per correct answer across all lessons.
pub fn total_stars(conn: &Connection, profile_id: i64) -> Result<usize, StoreError> {
    let n: i64 = conn.query_row(
        "SELECT COALESCE(SUM(correct_count), 0) FROM progress WHERE profile_id = ?1",
        params![profile_id],
        |row| row.get(0),
    )?;
    Ok(n.max(0) as usize)
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DayStat {
    /// Vietnam calendar day, e.g. "2026-09-07".
    pub day: String,
    /// Lessons first answered correctly on that day.
    pub lessons_done: i64,
    pub answers: i64,
}

/// Per-day activity for a profile, most recent day first, read from the
/// per-answer log so each attempt stays on the day it happened.
///
/// Days are Vietnam days even though `at` is stored in UTC; `days` counts
/// distinct days in the result, not a window.
pub fn daily_activity(
    conn: &Connection,
    profile_id: i64,
    days: i64,
) -> Result<Vec<DayStat>, StoreError> {
    let sql = format!(
        "WITH events AS (
             SELECT date(at, '{VN_OFFSET}') AS d, lesson_id, correct
             FROM answer_events WHERE profile_id = ?1
         ),
         first_done AS (
             SELECT lesson_id, MIN(d) AS d FROM events WHERE correct = 1 GROUP BY lesson_id
         )
         SELECT e.d AS day,
                (SELECT COUNT(*) FROM first_done f WHERE f.d = e.d) AS lessons_done,
                COUNT(*) AS answers
         FROM events e
         GROUP BY e.d
         ORDER BY e.d DESC
         LIMIT ?2"
    );
    let mut stmt = conn.prepare(&sql)?;
    let rows = stmt.query_map(params![profile_id, days], |row| {
        Ok(DayStat {
            day: row.get(0)?,
            lessons_done: row.get(1)?,
            answers: row.get(2)?,
        })
    })?;
    let mut out = Vec::new();
    for row in rows {
        out.push(row?);
    }
    Ok(out)
}

pub fn upsert_user(conn: &Connection, google_sub: &str, email: &str) -> Result<User, StoreError> {
    conn.execute(
        "INSERT INTO users (google_sub, email) VALUES (?1, ?2)
         ON CONFLICT(google_sub) DO UPDATE SET email = excluded.email",
        params![google_sub, email],
    )?;
    conn.query_row(
        "SELECT id, google_sub, email FROM users WHERE google_sub = ?1",
        params![google_sub],
        |row| {
            Ok(User {
                id: row.get(0)?,
                google_sub: row.get(1)?,
                email: row.get(2)?,
            })
        },
    )
    .map_err(StoreError::from)
}

pub fn add_profile(
    conn: &Connection,
    user_id: i64,
    name: &str,
    avatar_key: &str,
) -> Result<Profile, StoreError> {
    let exists: Option<i64> = conn
        .query_row(
            "SELECT id FROM users WHERE id = ?1",
            params![user_id],
            |row| row.get(0),
        )
        .optional()?;
    if exists.is_none() {
        return Err(StoreError::UserNotFound);
    }

    let count: i64 = conn.query_row(
        "SELECT COUNT(*) FROM profiles WHERE user_id = ?1",
        params![user_id],
        |row| row.get(0),
    )?;
    if count >= MAX_PROFILES_PER_USER as i64 {
        return Err(StoreError::ProfileLimit);
    }

    conn.execute(
        "INSERT INTO profiles (user_id, name, avatar_key, sort_order)
         VALUES (?1, ?2, ?3, ?4)",
        params![user_id, name, avatar_key, count],
    )?;
    let id = conn.last_insert_rowid();
    Ok(Profile {
        id,
        user_id,
        name: name.to_string(),
        avatar_key: avatar_key.to_string(),
        sort_order: count,
    })
}

pub fn list_profiles(conn: &Connection, user_id: i64) -> Result<Vec<Profile>, StoreError> {
    let mut stmt = conn.prepare(
        "SELECT id, user_id, name, avatar_key, sort_order
         FROM profiles WHERE user_id = ?1 ORDER BY sort_order, id",
    )?;
    let rows = stmt.query_map(params![user_id], |row| {
        Ok(Profile {
            id: row.get(0)?,
            user_id: row.get(1)?,
            name: row.get(2)?,
            avatar_key: row.get(3)?,
            sort_order: row.get(4)?,
        })
    })?;
    let mut profiles = Vec::new();
    for row in rows {
        profiles.push(row?);
    }
    Ok(profiles)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn user(conn: &Connection) -> User {
        upsert_user(conn, "google-sub-1", "parent@example.com").unwrap()
    }

    #[test]
    fn upsert_user_is_stable_on_google_sub() {
        let conn = open_memory().unwrap();
        let first = upsert_user(&conn, "sub-a", "a@example.com").unwrap();
        let second = upsert_user(&conn, "sub-a", "a-new@example.com").unwrap();
        assert_eq!(first.id, second.id);
        assert_eq!(second.email, "a-new@example.com");
    }

    #[test]
    fn two_profiles_ok_third_rejected() {
        let conn = open_memory().unwrap();
        let parent = user(&conn);

        let one = add_profile(&conn, parent.id, "An", "robot").unwrap();
        let two = add_profile(&conn, parent.id, "Binh", "cat").unwrap();
        let third = add_profile(&conn, parent.id, "Chi", "bear");

        assert_eq!(one.name, "An");
        assert_eq!(two.avatar_key, "cat");
        assert_eq!(third, Err(StoreError::ProfileLimit));
        assert_eq!(list_profiles(&conn, parent.id).unwrap().len(), 2);
    }

    #[test]
    fn profile_limit_is_per_user() {
        let conn = open_memory().unwrap();
        let a = upsert_user(&conn, "sub-a", "a@example.com").unwrap();
        let b = upsert_user(&conn, "sub-b", "b@example.com").unwrap();
        add_profile(&conn, a.id, "A1", "robot").unwrap();
        add_profile(&conn, a.id, "A2", "cat").unwrap();
        add_profile(&conn, b.id, "B1", "bear").unwrap();
        assert_eq!(list_profiles(&conn, a.id).unwrap().len(), 2);
        assert_eq!(list_profiles(&conn, b.id).unwrap().len(), 1);
    }

    #[test]
    fn add_profile_requires_user() {
        let conn = open_memory().unwrap();
        assert_eq!(
            add_profile(&conn, 99, "An", "robot"),
            Err(StoreError::UserNotFound)
        );
    }

    #[test]
    fn file_keeps_profiles_after_reopen() {
        let path = std::env::temp_dir().join(format!(
            "ho_tro_persist_{}_{}.sqlite",
            std::process::id(),
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_nanos()
        ));
        let _ = std::fs::remove_file(&path);

        {
            let conn = open_file(&path).unwrap();
            let parent = upsert_user(&conn, "sub-a", "a@example.com").unwrap();
            add_profile(&conn, parent.id, "An", "robot").unwrap();
        }

        {
            let conn = open_file(&path).unwrap();
            let parent = upsert_user(&conn, "sub-a", "a@example.com").unwrap();
            let profiles = list_profiles(&conn, parent.id).unwrap();
            assert_eq!(profiles.len(), 1);
            assert_eq!(profiles[0].name, "An");
        }

        let _ = std::fs::remove_file(&path);
    }

    #[test]
    fn record_answer_counts_and_marks_done() {
        let conn = open_memory().unwrap();
        let parent = user(&conn);
        let profile = add_profile(&conn, parent.id, "An", "robot").unwrap();

        let wrong = record_answer(&conn, profile.id, 1, false).unwrap();
        assert!(!wrong.done);
        assert_eq!(wrong.wrong_count, 1);

        let right = record_answer(&conn, profile.id, 1, true).unwrap();
        assert!(right.done);
        assert_eq!(right.correct_count, 1);
        assert_eq!(right.wrong_count, 1);

        // a wrong retry keeps the lesson done and only grows counters
        let again = record_answer(&conn, profile.id, 1, false).unwrap();
        assert!(again.done);
        assert_eq!(again.wrong_count, 2);
    }

    #[test]
    fn progress_is_per_profile() {
        let conn = open_memory().unwrap();
        let parent = user(&conn);
        let a = add_profile(&conn, parent.id, "An", "robot").unwrap();
        let b = add_profile(&conn, parent.id, "Binh", "cat").unwrap();
        record_answer(&conn, a.id, 1, true).unwrap();
        assert_eq!(progress_for_profile(&conn, a.id).unwrap().len(), 1);
        assert_eq!(progress_for_profile(&conn, b.id).unwrap().len(), 0);
    }

    #[test]
    fn next_lesson_skips_done() {
        let conn = open_memory().unwrap();
        let parent = user(&conn);
        let profile = add_profile(&conn, parent.id, "An", "robot").unwrap();
        let ids = [1u32, 2, 3];
        assert_eq!(next_lesson_id(&conn, profile.id, &ids).unwrap(), Some(1));
        record_answer(&conn, profile.id, 1, true).unwrap();
        assert_eq!(next_lesson_id(&conn, profile.id, &ids).unwrap(), Some(2));
        record_answer(&conn, profile.id, 2, true).unwrap();
        record_answer(&conn, profile.id, 3, true).unwrap();
        assert_eq!(next_lesson_id(&conn, profile.id, &ids).unwrap(), None);
    }

    #[test]
    fn daily_activity_groups_by_day() {
        let conn = open_memory().unwrap();
        let parent = user(&conn);
        let profile = add_profile(&conn, parent.id, "An", "robot").unwrap();
        record_answer(&conn, profile.id, 1, true).unwrap();
        record_answer(&conn, profile.id, 2, true).unwrap();
        record_answer(&conn, profile.id, 2, false).unwrap();
        let rows = daily_activity(&conn, profile.id, 7).unwrap();
        assert_eq!(rows.len(), 1);
        assert_eq!(rows[0].lessons_done, 2);
        assert_eq!(rows[0].answers, 3);
        assert_eq!(daily_activity(&conn, 999, 7).unwrap().len(), 0);
    }

    #[test]
    fn daily_activity_separates_the_days_answers_happened() {
        let conn = open_memory().unwrap();
        let parent = user(&conn);
        let profile = add_profile(&conn, parent.id, "An", "robot").unwrap();

        // Monday: lesson 1 wrong twice then right, lesson 2 right.
        record_answer_at(&conn, profile.id, 1, false, "2026-09-07 01:00:00").unwrap();
        record_answer_at(&conn, profile.id, 1, false, "2026-09-07 01:05:00").unwrap();
        record_answer_at(&conn, profile.id, 1, true, "2026-09-07 01:10:00").unwrap();
        record_answer_at(&conn, profile.id, 2, true, "2026-09-07 01:15:00").unwrap();
        // Tuesday: lesson 2 is answered again, which is a try but not a new lesson done.
        record_answer_at(&conn, profile.id, 2, false, "2026-09-08 01:00:00").unwrap();

        let rows = daily_activity(&conn, profile.id, 7).unwrap();
        assert_eq!(
            rows,
            vec![
                DayStat {
                    day: "2026-09-08".to_string(),
                    lessons_done: 0,
                    answers: 1,
                },
                DayStat {
                    day: "2026-09-07".to_string(),
                    lessons_done: 2,
                    answers: 4,
                },
            ]
        );
    }

    #[test]
    fn daily_activity_days_follow_vietnam_time() {
        let conn = open_memory().unwrap();
        let parent = user(&conn);
        let profile = add_profile(&conn, parent.id, "An", "robot").unwrap();

        // 23:00 UTC is 06:00 next day in Vietnam; the day flips at 17:00 UTC.
        record_answer_at(&conn, profile.id, 1, true, "2026-09-07 23:00:00").unwrap();
        record_answer_at(&conn, profile.id, 2, true, "2026-09-08 16:59:00").unwrap();
        record_answer_at(&conn, profile.id, 3, true, "2026-09-08 17:00:00").unwrap();

        let rows = daily_activity(&conn, profile.id, 7).unwrap();
        assert_eq!(
            rows,
            vec![
                DayStat {
                    day: "2026-09-09".to_string(),
                    lessons_done: 1,
                    answers: 1,
                },
                DayStat {
                    day: "2026-09-08".to_string(),
                    lessons_done: 2,
                    answers: 2,
                },
            ]
        );
    }

    #[test]
    fn failed_event_insert_rolls_back_progress() {
        let conn = open_memory().unwrap();
        let parent = user(&conn);
        let profile = add_profile(&conn, parent.id, "An", "robot").unwrap();

        // Force the second write to fail so the first one can be checked.
        conn.execute_batch("DROP TABLE answer_events;").unwrap();

        let result = record_answer_at(&conn, profile.id, 1, true, "2026-09-08 01:00:00");
        assert!(result.is_err());
        assert!(progress_for_profile(&conn, profile.id).unwrap().is_empty());
        assert_eq!(total_stars(&conn, profile.id).unwrap(), 0);
    }

    #[test]
    fn migrate_adds_answer_events_to_a_db_that_already_has_progress() {
        let path = std::env::temp_dir().join(format!(
            "ho_tro_migrate_{}_{}.sqlite",
            std::process::id(),
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_nanos()
        ));
        let _ = std::fs::remove_file(&path);

        {
            // The shipped schema before answer_events existed.
            let conn = Connection::open(&path).unwrap();
            conn.execute_batch(
                "CREATE TABLE users (
                    id INTEGER PRIMARY KEY,
                    google_sub TEXT NOT NULL UNIQUE,
                    email TEXT NOT NULL,
                    created_at TEXT NOT NULL DEFAULT (datetime('now'))
                );
                CREATE TABLE profiles (
                    id INTEGER PRIMARY KEY,
                    user_id INTEGER NOT NULL REFERENCES users(id) ON DELETE CASCADE,
                    name TEXT NOT NULL,
                    avatar_key TEXT NOT NULL,
                    sort_order INTEGER NOT NULL,
                    created_at TEXT NOT NULL DEFAULT (datetime('now'))
                );
                CREATE TABLE progress (
                    profile_id INTEGER NOT NULL REFERENCES profiles(id) ON DELETE CASCADE,
                    lesson_id INTEGER NOT NULL,
                    correct_count INTEGER NOT NULL DEFAULT 0,
                    wrong_count INTEGER NOT NULL DEFAULT 0,
                    done INTEGER NOT NULL DEFAULT 0,
                    last_tried_at TEXT NOT NULL DEFAULT (datetime('now')),
                    PRIMARY KEY (profile_id, lesson_id)
                );
                INSERT INTO users (id, google_sub, email) VALUES (1, 'sub-old', 'old@example.com');
                INSERT INTO profiles (id, user_id, name, avatar_key, sort_order)
                    VALUES (1, 1, 'An', 'robot', 0);
                INSERT INTO progress (profile_id, lesson_id, correct_count, wrong_count, done, last_tried_at)
                    VALUES (1, 5, 2, 1, 1, '2026-09-07 01:00:00');",
            )
            .unwrap();
        }

        {
            let conn = open_file(&path).unwrap();
            // Old rows survive the upgrade.
            let rows = progress_for_profile(&conn, 1).unwrap();
            assert_eq!(rows.len(), 1);
            assert!(rows[0].done);
            assert_eq!(total_stars(&conn, 1).unwrap(), 2);

            // And the new log is usable right away.
            record_answer_at(&conn, 1, 6, true, "2026-09-08 01:00:00").unwrap();
            let days = daily_activity(&conn, 1, 7).unwrap();
            assert_eq!(days.len(), 1);
            assert_eq!(days[0].day, "2026-09-08");
            assert_eq!(days[0].lessons_done, 1);
        }

        let _ = std::fs::remove_file(&path);
    }

    #[test]
    fn progress_survives_reopen() {
        let path = std::env::temp_dir().join(format!(
            "ho_tro_progress_{}_{}.sqlite",
            std::process::id(),
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_nanos()
        ));
        let _ = std::fs::remove_file(&path);

        {
            let conn = open_file(&path).unwrap();
            let parent = upsert_user(&conn, "sub-p", "p@example.com").unwrap();
            let profile = add_profile(&conn, parent.id, "An", "robot").unwrap();
            record_answer(&conn, profile.id, 5, true).unwrap();
        }

        {
            let conn = open_file(&path).unwrap();
            let parent = upsert_user(&conn, "sub-p", "p@example.com").unwrap();
            let profile = list_profiles(&conn, parent.id).unwrap().remove(0);
            let rows = progress_for_profile(&conn, profile.id).unwrap();
            assert_eq!(rows.len(), 1);
            assert!(rows[0].done);
        }

        let _ = std::fs::remove_file(&path);
    }
}
