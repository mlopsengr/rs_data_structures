#[derive(Debug)]
struct TableRow {
    name: String,
    age: i32,
    score: i32
}

struct Table {
    rows: Vec<TableRow>,
}

impl Table {

    fn new() ->Self {
        Table {rows: Vec::new() }
    }
    
    fn add_row(&mut self, row: TableRow) {
        self.rows.push(row);
    }

    fn print_table(&self) {
        println!("{:<15} {:<10} {:<10}", "Name", "Age", "Score");
        println!("{:-<15} {:-<10} {:-<10}", "", "", "");

        for row in &self.rows {
            println!("{:<15} {:<10} {:<10}", row.name, row.age, row.score);
        }
    }
}

fn main() {
    let mut my_table = Table::new();

    let row1 = TableRow {
        name: "Bob".to_string(),
        age: 42,
        score: 58
    };

    let row2 = TableRow {
        name: "Whitney".to_string(),
        age: 28,
        score: 65
    };

    my_table.add_row(row1);
    my_table.add_row(row2);

    my_table.print_table();
}
