extern crate rusqlite;
use std::os;
use std::io;
use rusqlite::SqliteConnection;

struct Todo {
	id: i64,
	title: String,
	description: String,
	complete: String
}

fn insert(conn: &SqliteConnection, todos: &mut Vec<Todo>) {
	println!("Name of item?");
	let title = io::stdin().read_line().ok().expect("Failed to read line");
	println!("Description?");
	let description = io::stdin().read_line().ok().expect("Failed to read line");
	let mut strmt = conn.execute("INSERT INTO todo (title, description, complete)
		          VALUES ($1, $2, $3)",
		          &[&title.trim(), &description.trim(), &"false"]).unwrap();

	todos.push(Todo{id: conn.last_insert_rowid(), title: title.trim().to_string(),
					description: description.trim().to_string(), complete: "False".to_string() });
}

fn list(conn: &SqliteConnection) -> Vec<Todo>{
	let mut todos = Vec::new();
	let mut stmt = conn.prepare("SELECT id, title, description, complete FROM todo WHERE complete = 'false'").unwrap();
	for row in stmt.query(&[]).unwrap().map(|row| row.unwrap()) {
		todos.push(Todo { id: row.get(0),
						  title: row.get(1),
						  description: row.get(2),
						  complete: row.get(3)});
	}

	todos
}

fn delete(conn: &SqliteConnection, todos: &mut Vec<Todo>) {
	println!("Which item would you like to delete?");
	let index = io::stdin().read_line().ok().expect("Failed to read line");
	let index_num: Option<usize> = index.trim().parse();
	let id = match index_num {
		Some(n) => n,
		None => {
			println!("Invalid number");
			return;
		}
	};
	
	let id_string = todos[id-1].id.to_string();
	conn.execute("UPDATE todo SET complete = 'true' WHERE id = $1",
				 &[&id_string.trim()]);
	todos.remove(id-1);
	println!("Todo item {} deleted", id);
}

fn display(todos: &Vec<Todo>) {
	let mut index = 1;
	for todo in todos.iter() {
		println!("{}. {}", index, todo.title);
		index += 1;
		
	}
}

fn detail(conn: &SqliteConnection, todos: &Vec<Todo>) {
	println!("Detail of which item?");
	let id = io::stdin().read_line().ok().expect("Failed to read line");
	let id_num: Option<usize> = id.trim().parse();
	match id_num {
		Some(n) => println!("{}", todos[n-1].description),
		None => {
			println!("Invalid entry")
		}
	}
}

fn menu(conn: &SqliteConnection) {
	let mut todos = list(conn);
	let mut exit = false;

	while !exit {
		display(&todos);
		println!("What is your command?");
		let mut command = io::stdin().read_line().ok().expect("Failed to read line");
		// command = command.trim();
		let commands: Vec<&str> = command.split(' ').collect();
		match commands[0].trim() {
			"quit" => exit = true,
			"add" => insert(conn, &mut todos),
			"detail" => detail(conn, &todos),
			"delete" => delete(conn, &mut todos),
			_ => exit = false
		}
	}
}

fn main() {
	let home_path = os::homedir();
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
	menu(&conn);
	// match args.len() {
	// 	1 => println!("Please provide an argument."),
	// 	_ => match args[1].as_slice() {
	// 		"add"  => insert(conn),
	// 		"list" => list(conn),
	// 		"delete" => delete(conn),
	// 		_      => println!("Invalid arguments")
	// 	}
	// }
}
