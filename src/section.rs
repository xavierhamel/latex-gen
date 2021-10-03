use crate::table;

pub struct Section {
    table: table::Table,
    name:String,
    points:u8,
    inside_table:bool,
}

impl Section {
    pub fn new(line:String) -> Self {
        Section {
            table:table::Table::new(),
            name:Section::parse_name(line),
            points:0,
            inside_table:false,
        }
    }

    fn parse_name(line:String) -> String {
        line[11..].trim().to_string()
    }

    pub fn parse_line(&mut self, line:String) {
        if self.inside_table || line.starts_with('|') {
            self.table.parse_line(line);
        }
    }

    pub fn generate(&self) {
        let mut output = String::from(
            format!("\\item \\textbf{{{}}}\n", self.name)
        );
        if self.table.is_table {
            output.push_str(&self.table.generate(&self.name));
        }

        println!("{}", output);
    }
}