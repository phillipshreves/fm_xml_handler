use quick_xml::Reader;
use quick_xml::events::Event;
use std::io::Read;

fn main(){
    let file = std::fs::File::open("/Users/phillipshreves/AltDesktop/Developer Documents/xml_parsing/xml_to_parse.xml");
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    let xml = contents;

    let mut reader = Reader::from_str(xml);
    reader.trim_text(true);

    let mut count = 0;
    let mut txt = Vec::new();
    let mut buf = Vec::new();

    // The `Reader` does not implement `Iterator` because it outputs borrowed data (`Cow`s)
    loop {
        match reader.read_event(&mut buf) {
        // for triggering namespaced events, use this instead:
        // match reader.read_namespaced_event(&mut buf) {
            Ok(Event::Start(ref e)) => {
            // for namespaced:
            // Ok((ref namespace_value, Event::Start(ref e)))
                match e.name() {
                    b"tag1" => println!("attributes values: {:?}",
                                        e.attributes().map(|a| a.unwrap().value)
                                        .collect::<Vec<_>>()),
                    b"tag2" => count += 1,
                    _ => (),
                }
            },
            // unescape and decode the text event using the reader encoding
            Ok(Event::Text(e)) => txt.push(e.unescape_and_decode(&reader).unwrap()),
            Ok(Event::Eof) => break, // exits the loop when reaching end of file
            Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
            _ => (), // There are several other `Event`s we do not consider here
        }

        // if we don't keep a borrow elsewhere, we can clear the buffer to keep memory usage low
        buf.clear();
    }
}