# fm_xml_handler

## Purpose
An XML parser that is intended to be used in conjuction with a FileMaker Pro database. It processes database records in the XML format "FMPXMLRESULT" to combine multiple tables into one XML file which can then be used in a single Import Records script step in FileMaker. 

## Result
FileMaker Pro can be slow when matching record data on an Import Records script step, so this was intended to bypass FileMaker's XML parser in hopes of improved performance. Unfortunately, it isn't as performant as expected, and is left here for historical purposes.

## Conclusion
A signficant improvement would be had by implementing a multi-threaded approach, however, FileMaker Pro's import process has proven to be very efficient. When processing Join tables and matching on key fields, FileMaker's process is extremely slow, but this is an understandable issue and can be avoided with proper pre-processing.
