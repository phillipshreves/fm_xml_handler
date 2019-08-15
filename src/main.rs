extern crate minidom;

use minidom::Element;
use std::collections::HashMap;
use std::fs;

#[derive(Debug)]
pub struct Field {
    empty_ok: String,
    max_repeat: String,
    name: String,
    field_type: String,
}

fn main() -> std::io::Result<()> {
    let mut record_hashmap = HashMap::new();
    let mut fields = Vec::new();

    let table_names = vec![String::from("Contacts"), String::from("ContactDetails")];
    let export_folder = String::from("/Users/phillipshreves/Desktop/");

    let mut counter_tables = 0;
    record_hashmap = loop {
        let table_name = &table_names[counter_tables];
        let filepath = String::from([&export_folder, "export_", table_name, ".xml"].concat());

        record_hashmap = update_record_hashmap(&filepath, record_hashmap);

        counter_tables += 1;
        if counter_tables >= table_names.len() {
            break record_hashmap;
        }
    };

    let mut counter_tables = 0;
    fields = loop {
        let table_name = &table_names[counter_tables];
        let filepath = String::from([&export_folder, "export_", &table_name, ".xml"].concat());

        fields = field_metadata(&filepath, &table_name, fields);

        counter_tables += 1;
        if counter_tables >= table_names.len() {
            break fields;
        }
    };

    //Example Results:
    //println!("{:#?}", record_hashmap["19991"]);
    //println!("{:#?}", fields[0]);
    //println!("{:#?}", xml_field_metadata(fields));
    //println!("{}", xml_record_data(record_hashmap));

    let xml_to_write = format!("<?xml version=\"1.0\" encoding=\"UTF-8\" ?>
<FMPXMLRESULT xmlns=\"http://www.filemaker.com/fmpxmlresult\">
    <ERRORCODE>0</ERRORCODE>
    <PRODUCT BUILD=\"07-05-2019\" NAME=\"FileMaker\" VERSION=\"ProAdvanced 18.0.2\"/>
    <DATABASE DATEFORMAT=\"M/d/yyyy\" LAYOUT=\"\" NAME=\"TM_Dev.fmp12\" RECORDS=\"53757\" TIMEFORMAT=\"h:mm:ss a\"/>
    <METADATA>
    {}
    </METADATA>
    <RESULTSET>
    {}
    </RESULTSET>
</FMPXMLRESULT>", xml_field_metadata(fields), xml_record_data(record_hashmap)) ;

    fs::write(format!("{}import_test.xml", export_folder), xml_to_write)?;
    Ok(())
}

fn field_metadata(
    xml_filepath: &String,
    table_name: &String,
    mut fields: Vec<Field>,
) -> Vec<Field> {
    // The fields vector will contain all of the metadata for the fields in order so that when we pull them later we can line them up with the data
    /*
    let field_example = Field {
        empty_ok: String::from("YES"),
        max_repeat: String::from("1"),
        name: String::from("ID_Contact"),
        field_type: String::from("NUMBER"),
    };
    fields.push(field_example);
    */
    let xml = fs::read_to_string(xml_filepath).expect("file error");

    let root: Element = xml.parse().unwrap();
    for table in root.children() {
        if table.name() == "METADATA" {
            for field in table.children() {
                let mut field_structure = Field {
                    empty_ok: String::new(),
                    field_type: String::new(),
                    max_repeat: String::new(),
                    name: String::new(),
                };
                for attribute in field.attrs() {
                    let a_name = attribute.0;
                    let a_value = attribute.1;
                    match a_name {
                        "EMPTYOK" => field_structure.empty_ok = String::from(a_value),
                        "MAXREPEAT" => field_structure.max_repeat = String::from(a_value),
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
                        let data = value.text();
                        record_data.push(data);
                    }
                }
                let record_id = record_data[0].clone();
                let mut record_vec = Vec::new();
                if record_hashmap.contains_key(&record_id) {
                    record_vec.extend(record_hashmap[&record_id].clone());
                };
                record_vec.extend(record_data);
                record_hashmap.insert(record_id, record_vec);
            }
        }
    }

    record_hashmap
}

fn xml_field_metadata(field_vector: Vec<Field>) -> String {
    //This sets up the XML which determines what field contains the data found in xml_record_data
    let mut xml = String::new();

    for field in &field_vector {
        xml = format!(
            "{}<FIELD EMPTYOK=\"{}\" MAXREPEAT=\"{}\" NAME=\"{}\" TYPE=\"{}\"/>",
            xml, field.empty_ok, field.max_repeat, field.name, field.field_type
        );
    }

    xml
}

fn xml_record_data(record_data: HashMap<String, Vec<String>>) -> String {
    //This sets up the XML for the record data
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
