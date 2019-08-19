extern crate minidom;

use minidom::Element;
use std::collections::HashMap;
use std::fs;
use std::env;

#[derive(Debug)]
pub struct Field {
    name: String,
    field_type: String,
}

//These static variables will be used to identify escape characters
static AMPERSAND: u8 = '&' as u8;
static LESS_THAN: u8 = '<' as u8;
static GREATER_THAN: u8 = '>' as u8;
static QUOTE_DOUBLE: u8 = '\"' as u8;
static QUOTE_SINGLE: u8 = '\'' as u8;

fn main() -> std::io::Result<()> {
//These parameters are set at runtime through arguments
    //Argument 1 - export folder - "/Users/phillipshreves/Desktop/"
    //Argument 2 - table names - "Contacts|ContactDetails"
    let arguments: Vec<String> = env::args().collect();
    let export_folder = &arguments[1];
    let table_names_arg = &arguments[2];
    let table_names: Vec<&str> = table_names_arg.as_str().split('|').collect();

//Variable setup
    let mut record_hashmap = HashMap::new();
    let mut fields = Vec::new();

//Now we get the record data, parse it out, and build the xml
    let mut counter_tables = 0;
    record_hashmap = loop {
        let table_name = String::from(table_names[counter_tables]);
        let filepath = format!("{}{}{}", &export_folder, &table_name, ".xml");

        record_hashmap = update_record_hashmap(&filepath, record_hashmap);

        counter_tables += 1;
        if counter_tables >= table_names.len() {
            break record_hashmap;
        }
    };

//Now we get the field metadata, parse it out, and build the xml
    let mut counter_tables = 0;
    fields = loop {
        let table_name = String::from(table_names[counter_tables]);
        let filepath = format!("{}{}{}", &export_folder, &table_name, ".xml");

        fields = field_metadata(&filepath, &table_name, fields);

        counter_tables += 1;
        if counter_tables >= table_names.len() {
            break fields;
        }
    };

//Create the XML
    let xml_to_write = format!("<?xml version=\"1.0\" encoding=\"UTF-8\" ?>
<FMPXMLRESULT xmlns=\"http://www.filemaker.com/fmpxmlresult\">
    <METADATA>
    {}
    </METADATA>
    <RESULTSET>
    {}
    </RESULTSET>
</FMPXMLRESULT>", xml_field_metadata(fields), xml_record_data(record_hashmap)) ;

//Write XML to file
    fs::write(format!("{}data_to_import.xml", export_folder), xml_to_write)?;
    Ok(())
}

fn escape_xml(data: String) -> String {
    //Check if we need to escape any characters, return early if we don't.
    let mut escape_characters = false;
    if data.contains("&")
        | data.contains("<")
        | data.contains(">")
        | data.contains("\"")
        | data.contains("\'")
    {
        escape_characters = true;
    };
    if !escape_characters {
        return data;
    }

    //change character to escape string
    let mut data_escaped = String::new();
    for character in data.chars(){
        let char_u8 = character as u8 ;
        let mut character_escaped = character.to_string();
        if char_u8 == AMPERSAND {
            character_escaped = "&amp;".to_string();
        } else if char_u8 == LESS_THAN {
            character_escaped = "&lt;".to_string();
        } else if char_u8 == GREATER_THAN {
            character_escaped = "&gt;".to_string();
        } else if char_u8 == QUOTE_DOUBLE {
            character_escaped = "&quot;".to_string();
        } else if char_u8 == QUOTE_SINGLE {
            character_escaped = "&apos;".to_string();
        };
        data_escaped = format!("{}{}", data_escaped, character_escaped);
    };

    return data_escaped;
}

fn field_metadata(
    xml_filepath: &String,
    table_name: &String,
    mut fields: Vec<Field>,
) -> Vec<Field> {
    // The fields vector will contain all of the metadata for the fields in order so that when we pull them later we can line them up with the data /*
    let xml = fs::read_to_string(xml_filepath).expect("file error");

    let root: Element = xml.parse().unwrap();
    for table in root.children() {
        if table.name() == "METADATA" {
            for field in table.children() {

                let mut field_structure = Field {
                    field_type: String::new(),
                    name: String::new(),
                };

                for attribute in field.attrs() {
                    let a_name = attribute.0;
                    let a_value = attribute.1;
                    match a_name {
                        "NAME" => {
                            field_structure.name =
                                String::from([table_name, "__", a_value].concat())
                        }
                        "TYPE" => field_structure.field_type = String::from(a_value),
                        _ => (),
                    }
                }

                fields.push(field_structure);
            }
        }
    }

    fields
}

fn update_record_hashmap(
    xml_filepath: &String,
    mut record_hashmap: HashMap<String, Vec<String>>,
) -> HashMap<String, Vec<String>> {
    // The records hashmap will contain the records using the primary key from the table, and then holding the field data in a vector for each
    let xml = fs::read_to_string(xml_filepath).expect("file error");

    let root: Element = xml.parse().unwrap();
    for table in root.children() {
        if table.name() == "RESULTSET" {
            for record in table.children() {

                let mut record_data = Vec::new();
                
                for field in record.children() {
                    for value in field.children() {
                        let mut data = value.text();
                        data = escape_xml(data);
                        record_data.push(data);
                    }
                }

                let record_id = record_data[0].clone();
                let mut record_vec = Vec::new();
                if record_hashmap.contains_key(&record_id) {
                    record_vec = record_hashmap[&record_id].clone();
                };
                record_vec.extend(record_data);
                record_hashmap.insert(record_id, record_vec);
            }
        }
    }

    record_hashmap
}

fn xml_field_metadata(field_vector: Vec<Field>) -> String {
    //This sets up the METADATA XML which determines what field the data pertains to
    let mut xml = String::new();

    for field in &field_vector {
        xml = format!(
            "{}<FIELD NAME=\"{}\" TYPE=\"{}\"/>",
            xml, field.name, field.field_type
        );
    }

    xml
}

fn xml_record_data(record_data: HashMap<String, Vec<String>>) -> String {
    //This sets up the RESULTSET XML for the record data
    let mut xml = String::new();

    for record in record_data.values() {
        let mut record_values = String::new();
        for value in record {
            record_values = format!("{}<COL><DATA>{}</DATA></COL>", record_values, value);
        }
        xml = format!("{}<ROW>{}</ROW>", xml, record_values)
    }

    xml
}
