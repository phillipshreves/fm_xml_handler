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
    let record_hashmap = HashMap::new();

    let filepath = String::from("/Users/phillipshreves/Desktop/export_contacts.xml");
    let record_hashmap = record_data_hashmap(filepath, record_hashmap);
    let filepath = String::from("/Users/phillipshreves/Desktop/export_contactdetails.xml");
    let record_hashmap = record_data_hashmap(filepath, record_hashmap);

       // The fields vector will contain all of the metadata for the fields in order so that when we pull them later we can line them up with the data
    let mut fields: Vec<Field> = Vec::new();
    let field_example = Field {
        empty_ok: String::from("YES"),
        max_repeat: String::from("1"),
        name: String::from("ID_Contact"),
        field_type: String::from("NUMBER"),
    };
    fields.push(field_example);

    println!("{:#?}", record_hashmap);
    return ;
}

fn record_data_hashmap(xml_filepath: String, mut record_hashmap: HashMap<String, Vec<String>>) -> HashMap<String, Vec<String>>{
 
    let xml = fs::read_to_string(xml_filepath).expect("file error");
    // The records hashmap will contain the records using the primary key from the table, and then holding the field data in a vector for each

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
                let record_id = record_data[0].clone();
                record_hashmap.entry(record_id).or_insert(record_data);
            }
        }
    }

    record_hashmap
}


fn field_metadata(xml_filepath: String, mut field_vec: Vec<String>) -> Vec<String>{
 
    let xml = fs::read_to_string(xml_filepath).expect("file error");
    // The records hashmap will contain the records using the primary key from the table, and then holding the field data in a vector for each

    let root: Element = xml.parse().unwrap();
    for table in root.children() {
        if table.name() == "METADATA" {
            for field in table.children() {
                
            }
        }
    }

    field_vec
}