extern crate quick_xml;

use quick_xml::Reader;
use quick_xml::events::Event;

fn main() {
    let xml = r#"<tag1 att1 = "test">
                    <tag2><!--Test comment-->Test</tag2>
                    <tag2>
                        Test 2
                    </tag2>
                </tag1>"#;
    let mut reader = Reader::from_str(xml);
    reader.trim_text(true);

}

fn stream(xml: str){
    println!("{}", String::from_str(xml));
}