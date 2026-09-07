use rusqlite::{params, Connection, OptionalExtension};
use std::path::Path;

pub const MAX_PROFILES_PER_USER: usize = 2;

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
        ",
    )?;
    Ok(())
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
}
