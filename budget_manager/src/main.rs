use rusqlite::{params , Connection , Result};
use std::time::{SystemTime, UNIX_EPOCH};

struct Transaction {
    amount: i64,
    category: String,
    date: i64,
    description: String
}

impl Transaction {
    fn new(amount: i64, category: String, date: i64, description: String) -> Transaction {
        Transaction { amount, category, date, description }
    }
}

fn main() -> Result<()> {
    let connection = Connection::open("test.db")?;

    // connection.execute("DROP TABLE trans", ());

    // let result = connection.execute("
    //     CREATE TABLE IF NOT EXISTS trans (
    //         amount INTEGER,
    //         date INTEGER,
    //         description TEXT,
    //         category TEXT
    //     )
    // " , ());

    let transaction1 = Transaction::new(10000 , String::from("Food") , SystemTime::now().duration_since(UNIX_EPOCH).expect("Time went backward").as_secs() as i64 , String::from("Bought Cheese"));

    add_transaction(transaction1, &connection);

    let amount = get_current_amount(&connection).unwrap_or(0);

    println!("{}" , amount);




    Ok(())
}

fn add_transaction(transaction: Transaction , connection: &Connection) {
    let _ = connection.execute("
        INSERT INTO trans (amount, date, description, category) VALUES (?1, ?2, ?3, ?4)

    ", params![transaction.amount, transaction.date, &transaction.description, &transaction.category]);
}

fn get_current_amount(connection: &Connection) -> Result<i64> {
    let mut stmt = connection.prepare("SELECT SUM(amount) FROM trans")?;

    let amount: i64 = stmt.query_row([] , |row| {
        let value: Option<i64> = row.get(0)?;
        Ok(value.unwrap_or(0))
    })?;

    // println!("{}" , result.unwrap() , |row| row);
    Ok(amount)
}
