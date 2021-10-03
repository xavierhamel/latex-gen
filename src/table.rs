use crate::cell;

enum TableState {
    Header,
    Body,
    Example,
    Incertitude,
}

pub struct Table {
    pub header_cells:Vec<cell::HeaderCell>,
    pub body_cells:Vec<cell::BodyCell>,
    pub example_cells:Vec<cell::BodyCell>,
    pub incertitude_cells:Vec<cell::BodyCell>,
    pub is_table:bool,
    rows:usize,
    state:TableState,
}

impl Table {
    pub fn new() -> Self {
        Table {
            header_cells:vec![],
            body_cells:vec![],
            example_cells:vec![],
            incertitude_cells:vec![],
            is_table:false,
            rows:0,
            state:TableState::Header,
        }
    }

    pub fn parse_line(&mut self, line:String) -> bool {
        match line.chars().next() {
            Some('|') => {
                let mut col = 0;
                for cell in line.split('|') {
                    match self.state {
                        TableState::Header => {
                            if cell.trim().len() > 0 {
                                self.header_cells.push(cell::HeaderCell::new(cell, col));
                                self.is_table = true;
                            }
                        },
                        TableState::Body => {
                            if col > 0 && col < self.header_cells.len() + 1 {
                                self.body_cells.push(cell::BodyCell::new_body(cell, col, &self.header_cells));
                            } else if col > 0 && cell.len() > 0 {
                                if cell.chars().next().unwrap() == '*' {
                                    self.rows = cell[1..].parse::<usize>().unwrap();
                                }
                            }
                        },
                        TableState::Example => {
                            if col > 0 && col < self.header_cells.len() + 1 {
                                self.example_cells.push(cell::BodyCell::new_example(cell, col));
                            }
                        },
                        TableState::Incertitude => {
                            if col > 0 && col < self.header_cells.len() + 1 {
                                self.incertitude_cells.push(cell::BodyCell::new(cell, col));
                            }
                        },
                    }
                    col += 1;
                }
                self.state = match self.state {
                    TableState::Header => TableState::Body,
                    TableState::Body => TableState::Example,
                    TableState::Example => TableState::Incertitude,
                    TableState::Incertitude => TableState::Incertitude,
                };
                true
            },
            _ => false
        }
    }

    pub fn generate(&self, name:&String) -> String {
        let mut output = String::from("");
        output.push_str(&self.generate_header(name));
        output.push_str(&self.generate_body());
        output.push_str(&self.generate_footer());
        output.push_str(&self.generate_example());
        output
    }

    fn generate_header(&self, name:&String) -> String {
        let mut output = String::from("");
        output.push_str(
            &format!("\t\\begin{{figure}}[hbt!]\n\t\t\\centering\n\t\t\\captionof{{table}}{{{}}}\n\t\t\\begin{{spreadtab}}{{{{tabu}}{{", name)
        );
        output.push_str(&std::iter::repeat("|c").take(self.header_cells.len()).collect::<String>());
        output.push_str("|}}\\hline\n\t\t\t");
        let units = std::iter::repeat("@$   $").take(self.header_cells.len()).collect::<Vec<&str>>().join(" & ");
        for cell in &self.header_cells {
            output.push_str(&format!("@$ {} $ ", cell.name));
            if cell.col < self.header_cells.len() {
                output.push_str("& ");
            }
        }
        output.push_str("\\\\\\hline\n\t\t\t");
        output.push_str(&units);
        output.push_str(" \\\\\\hline\\hline\n\t\t\t");
        output
    }

    fn generate_footer(&self) -> String {
        let mut output = String::from("");
        output.push_str("\n\t\t\\end{spreadtab}\n\t\\end{figure}\n");
        output

    }

    fn generate_body(&self) -> String {
        let mut output = String::from("");
        for row in 0..self.rows {
            for cell in &self.body_cells {
                output.push_str(&format!("{} ", cell.compute_formula(row)));
                if cell.col < self.body_cells.len() {
                    output.push_str("& ");
                }
            }
            output.push_str("\\\\\\hline");
            if row < self.rows - 1 {
                output.push_str("\n\t\t\t");
            }
        }
        output
    }

    fn generate_example(&self) -> String {
        let mut output = String::from("");
        for example in &self.example_cells {
            if example.formula.is_some() {
                output.push_str(&format!(
                    "\n\tExemple de calcul pour ${}$:\n\t\\[ {} = {} = \\]\n",
                    self.header_cells[example.col - 1].name,
                    self.header_cells[example.col - 1].name,
                    example.formula.clone().unwrap(),
                    // example.compute_example(self.header_cells.clone(), self.example_cells.clone()),
                ));
            }
        }
        output
    }

    // fn generate_incertitude(&self) -> String {
    //     let mut output = String::from("");
    //     for example in &self.example_cells {
    //         if example.formula.len() > 0 {
    //             output.push_str(&format!(
    //                 "\nExemple de calcul pour ${}$:\\\\\n \\[ {} = \\]\n",
    //                 self.header_cells[example.col - 1].name,
    //                 example.formula,
    //             ));
    //         }
    //     }
    //     output
    // }
}