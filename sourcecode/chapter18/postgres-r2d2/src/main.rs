extern crate postgres;
extern crate r2d2;
extern crate r2d2_postgres;
extern crate uuid;
#[macro_use]
extern crate serde_json;
extern crate chrono;

use r2d2_postgres::{TlsMode, PostgresConnectionManager};
use chrono::offset::utc::UTC;
use std::{thread, time};
use std::thread::sleep;

#[derive(Debug)]
struct Teacher {
    name: String,
    regtime: String,
    data: String
}

type DbConnection = r2d2::PooledConnection<PostgresConnectionManager>;

fn create_table(conn: DbConnection) {
    conn.execute("CREATE TABLE IF NOT EXISTS teacher (
                    id              SERIAL PRIMARY KEY,
                    name            VARCHAR NOT NULL,
                    regtime         VARCHAR NOT NULL,
                    data            VARCHAR NOT NULL
                  )", &[]).unwrap();
}

fn insert_teacher(conn: DbConnection) {
    let me = Teacher {
        name: "Milly".to_string(),
        regtime: UTC::now().to_string(),
        data: json!({
            "tags": ["employee", "future_prof"],
            "contacts": {
                "email": "milly@sztu.com"
            }
        }).to_string()
    };
    conn.execute("INSERT INTO teacher (name, data, regtime) VALUES ($1, $2, $3)",
                 &[&me.name, &me.data, &me.regtime]).expect("Table creation");
    
}

fn main() {
    let config = r2d2::Config::builder()
        .pool_size(5)
        .build();
    let manager = PostgresConnectionManager::new("postgres://postgres:123456@47.104.153.34:13025/rustbook", TlsMode::None).unwrap();
    let pool = r2d2::Pool::new(config, manager).unwrap();

    {
        let pool = pool.clone();
        thread::spawn(move || {
            let conn = pool.get().unwrap();
            create_table(conn);
            println!("Table creation thread finished.");
        });
    }

    {
        let pool = pool.clone();
        thread::spawn(move || {
            sleep(time::Duration::from_millis(500));
            let conn = pool.get().unwrap();
            insert_teacher(conn);
            println!("Teacher insertion thread finished.");
        });
    }

    sleep(time::Duration::from_millis(1000));
    {
        for _ in 0..1024 {
            let pool = pool.clone();
            thread::spawn(move || {
                let conn = pool.get().unwrap();
                for row in &conn.query("SELECT id, name, data, regtime FROM teacher", &[]).unwrap() {
                    let teacher = Teacher {
                        name: row.get(1),
                        data: row.get(2),
                        regtime: row.get(3)
                    };
                    println!("Found teacher {:?}", teacher);
                }
            });
        }
    }
}


