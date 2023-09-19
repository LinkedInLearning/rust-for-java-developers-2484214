use std::collections::HashMap;
use std::io::{Read, Result};
use std::str::FromStr;

#[allow(dead_code)]
#[derive(Debug)]
struct Report {
    num_rows: usize,
    num_fields: usize,
    num_missing_fields: usize,
    sum_of_numeric_fields: f64,
    length_of_text_fields: usize,
}

struct TypedCsv {
    rows: Vec<HashMap<String, Option<Field>>>,
}

enum Field {
    Text(String),
    Number(f64),
}

impl FromStr for Field {
    type Err = ();

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        if s.is_empty() {
            return Err(());
        }

        if let Ok(n) = s.parse::<f64>() {
            Ok(Field::Number(n))
        } else {
            Ok(Field::Text(s.to_string()))
        }
    }
}

fn parse_csv<T: Read>(mut source: T) -> Result<TypedCsv> {
    let mut raw_data = String::new();
    source.read_to_string(&mut raw_data)?;

    let mut headers = Vec::new();
    let mut rows = Vec::new();

    for (i, line) in raw_data.lines().enumerate() {
        if i == 0 {
            for header in line.split(',') {
                headers.push(header.to_string());
            }
            continue;
        }

        let mut row = HashMap::with_capacity(headers.len());

        for (header, raw_field) in headers.iter().zip(line.split(',')) {
            let field = raw_field.parse::<Field>().ok();
            row.insert(header.clone(), field);
        }

        if row.len() != headers.len() {
            continue;
        }

        rows.push(row);
    }

    Ok(TypedCsv { rows })
}

fn generate_report(csv: &TypedCsv) -> Report {
    let mut rep = Report {
        num_rows: 0,
        num_fields: 0,
        num_missing_fields: 0,
        sum_of_numeric_fields: 0.0,
        length_of_text_fields: 0,
    };

    for row in csv.rows.iter() {
        rep.num_rows += 1;
        if rep.num_fields == 0 {
            rep.num_fields = row.len();
        }
        for field in row.values() {
            match field {
                Some(Field::Text(t)) => rep.length_of_text_fields += t.len(),
                Some(Field::Number(n)) => rep.sum_of_numeric_fields += n,
                None => rep.num_missing_fields += 1,
            }
        }
    }

    rep
}

fn main() {
    let data: &[u8] = include_bytes!("dogs.txt");
    let csv = parse_csv(data).unwrap();

    println!("{:#?}", generate_report(&csv));
}
