extern crate rusqlite;
use std::os;
use std::io;
use rusqlite::SqliteConnection;

struct Todo {
	id: i32,
	title: String,
	description: String,
	complete: String
}

fn insert(conn: SqliteConnection) {
	println!("Name of item?");
	let title = io::stdin().read_line().ok().expect("Failed to read line");
	println!("Description?");
	let description = io::stdin().read_line().ok().expect("Failed to read line");
	conn.execute("INSERT INTO todo (title, description, complete)
		          VALUES ($1, $2, $3)",
		          &[&title.trim(), &description.trim(), &"false"]).unwrap();
}

fn list(conn: SqliteConnection) {
	let mut todos = Vec::new();
	let mut stmt = conn.prepare("SELECT id, title, description, complete FROM todo WHERE complete = 'false'").unwrap();
	for row in stmt.query(&[]).unwrap().map(|row| row.unwrap()) {
		todos.push(Todo { id: row.get(0),
						  title: row.get(1),
						  description: row.get(2),
						  complete: row.get(3)});
	}

	display(todos);
}


fn display(todos: Vec<Todo>) {
	let mut index = 0;
	for todo in todos.iter() {
		println!("{}. {}", index + 1, todo.title);
		index += 1;
		
	}
}

fn main() {

	let home_path = os::homedir();
	let stringy = "hoi";
	let mut db_path: String;
	match home_path {
		Some(path) => match path.as_str() {
			Some(path_s) => db_path = format!("file:{}/.todolist/todo.db", path_s),
			None => db_path = "file:todo.db".to_string()
		},
		None => db_path = "file:todo.db".to_string()
	}
	let conn = SqliteConnection::open(db_path.as_slice()).unwrap();

	conn.execute("CREATE TABLE IF NOT EXISTS todo (
				 id             INTEGER PRIMARY KEY,
				 title          TEXT NOT NULL,
				 description    TEXT NOT NULL,
				 complete       TEXT NOT NULL)", &[]).unwrap();
	let args = os::args();
	// match &os::args()[1] {
	// 	"hi" => println!("test"),
	// 	_ => println!("a")
	// }
	match args.len() {
		1 => println!("Please provide an argument."),
		_ => match args[1].as_slice() {
			"add"  => insert(conn),
			"list" => list(conn),
			_      => println!("Invalid arguments")
		}
	}
}
