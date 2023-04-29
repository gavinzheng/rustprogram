use postgres::{error::Error, Client, NoTls};

#[derive(Debug)]
struct ClassroomStudent {
    department: String,
    name: String,
    result: f64,
    comment: String,
    register_date: i64,
}

fn create_postgres_table() -> Result<Client, Error> {
    let username = "postgres";
    let password = "123456";
    let host = "47.104.153.34";
    let port = "13025";
    let database = "rustbook";
    let mut conn = Client::connect(
        &format!(
            "postgres://{}{}{}@{}{}{}{}{}",
            username,
            if password.is_empty() { "" } else { ":" },
            password,
            host,
            if port.is_empty() { "" } else { ":" },
            port,
            if database.is_empty() { "" } else { "/" },
            database
        ),
        NoTls,
    )?;
    let _ = conn.execute("DROP TABLE ClassroomStudent", &[]);
    let _ = conn.execute("DROP TABLE Student", &[]);
    conn.execute(
        "CREATE TABLE Student (
            id INTEGER PRIMARY KEY,
            department TEXT NOT NULL,
            name TEXT NOT NULL UNIQUE)",
        &[],
    )?;
    conn.execute(
        "CREATE TABLE ClassroomStudent (
            id TEXT PRIMARY KEY,
            student_id INTEGER NOT NULL REFERENCES Products,
            register_date BIGINT NOT NULL,
            result DOUBLE PRECISION NOT NULL,
            comment TEXT NOT NULL)",
        &[],
    )?;
    Ok(conn)
}

fn populate_table(conn: &mut Client) -> Result<(), Error> {
    conn.execute(
        "INSERT INTO Student (
            id, department, name
            ) VALUES ($1, $2, $3)",
        &[&1, &"computer science", &"Alice"],
    )?;
    conn.execute(
        "INSERT INTO ClassroomStudent (
            id, student_id, register_date, result, comment
            ) VALUES ($1, $2, $3, $4, $5)",
        &[&"2023-001", &1, &1_200_300_400_i64, &85.5, &"exam + experiment"],
    )?;
    Ok(())
}

fn query_db(conn: &mut Client) -> Result<(), Error> {
    for row in &conn.query(
        "SELECT s.name, c.comment, c.result, c.register_date
        FROM ClassroomStudent c
        LEFT JOIN Student s
        ON s.id = c.student_id
        ORDER BY c.register_date",
        &[],
    )? {
        let classroom_student = ClassroomStudent {
            department: "".to_string(),
            name: row.get(0),
            result: row.get(2),
            comment: row.get(1),
            register_date: row.get(3),
        };
        println!(
            "Student {}, register at {}, final result is  {} - {} .",
            classroom_student.name,
            classroom_student.register_date,
            classroom_student.result,
            classroom_student.comment,
        );
    }
    Ok(())
}

fn main() -> Result<(), Error> {
    let mut conn = create_postgres_table()?;
    populate_table(&mut conn)?;
    query_db(&mut conn)?;
    Ok(())
}
