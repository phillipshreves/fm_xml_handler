extern crate minidom;

use std::fs;
use std::collections::HashMap;
use minidom::Element;


/*
const DATA: &'static str = r#"<articles xmlns="article">
    <article>
        <title>10 Terrible Bugs You Would NEVER Believe Happened</title>
        <body>
            Rust fixed them all. &lt;3
        </body>
    </article>
    <article>
        <title>BREAKING NEWS: Physical Bug Jumps Out Of Programmer's Screen</title>
        <body>
            Just kidding!
        </body>
    </article>
</articles>"#;
*/

#[derive(Debug)]
pub struct Field {
    empty_ok: String,
    max_repeat: String,
    name: String,
    field_type: String,
}

fn main() {
    let filepath = "/Users/phillipshreves/Desktop/xml_test.xml";
    let xml = fs::read_to_string(filepath).expect("file error");

    // The fields vector will contain all of the metadata for the fields in order so that when we pull them later we can line them up with the data
    let mut fields: Vec<Field> = Vec::new();
    // The records hashmap will contain the records using the primary key from the table, and then holding the field data in a vector for each
    let mut records = HashMap::new();

    let root: Element = xml.parse().unwrap();
    for table in root.children() {
        if table.name() == "RESULTSET" {
            for record in table.children() {
                let mut record_data = Vec::new();
                for field in record.children() {
                    for value in field.children() {
                        let data = value.text();
                        record_data.push(data);
                    }
                }
                let record_id = &record_data;
                records.entry("test").or_insert(record_data);
            }
        }
    }

    println!("{:#?}", records);
    return ;
}
