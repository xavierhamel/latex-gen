
#[derive(Clone)]
pub struct HeaderCell {
    pub name:String,
    pub variable:String,
    pub col:usize,
}

#[derive(Clone)]
pub struct BodyCell {
    pub formula:Option<String>,
    pub value:Option<f64>,
    pub col:usize,
}

impl HeaderCell {

    pub fn new(line:&str, col:usize) -> Self {
        let name = line.trim().to_string();
        if name.len() == 1 {
            println!("ATTENTION: Le nom des colonnes d'une table devrait avoir plus d'un caractère. Pour avoir un nom de colonne avec seulement un caractère, ajouter un '_' après le caractère. Par exemple {} -> {}_", name, name);
        }
        let cols = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
        HeaderCell {
            name:line.trim().to_string(),
            variable:cols[col..col + 1].to_string(),
            col,
        }
    }
}

impl BodyCell {
    pub fn new(line:&str, col:usize) -> Self {
        BodyCell {
            formula:Some(line.trim().to_string()),
            value:None,
            col,
        }
    }

    pub fn new_body(line:&str, col:usize, headers:&Vec<HeaderCell>) -> Self {
        let mut formula = line.trim().to_string();
        let cols = "abcdefghijklmnopqrstuvwxyz";
        for (index, cell) in headers.iter().enumerate() {
            let replacement = format!("{}_", cols[index..index + 1].to_string());
            formula = formula.replace(&cell.name, &replacement);
        }
        BodyCell {
            formula: Some(formula),
            value:None,
            col,
        }
    }

    pub fn new_example(line:&str, col:usize) -> Self {
        let l = line.trim().to_string();
        let mut formula = None;
        let mut value = None;
        if l.starts_with("=") {
            formula = Some(l[1..].to_string());
        } else {
            if l.len() > 0 {
                value = Some(l.parse::<f64>().unwrap());
            } else {
                value = Some(0.0);
            }
        }
        BodyCell {
            formula,
            value,
            col,
        }
    }


    pub fn compute_formula(&self, row:usize) -> String {
        self.formula.clone().unwrap().replace("_", &(row + 3).to_string())
    }

    pub fn compute_example(&self, header:Vec<HeaderCell>, cells:Vec<BodyCell>) -> String {
        let mut output = self.formula.clone().unwrap();
        let mut i = 0;
        for cell in cells {
            if cell.value.is_some() {
                output = output.replace(&header[i].name, &cell.value.unwrap().to_string());
            }
            i += 1;
        }
        output
    }
}