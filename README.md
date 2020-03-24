# fm_xml_handler

An XML parser that is intended to be used in conjuction with a FileMaker Pro database. It processes database records in the XML format "FMPXMLRESULT" to combine multiple tables into one XML file which can then be used in single Import Records script step in FileMaker. 

FileMaker Pro can be slow when matching record data on an Import Records script step, so this was intended to bypass FileMaker's XML parser in hopes of improved performance. Unfortunately, it isn't as performant as expected, and is left here for historical purposes.

A signficant improvement would be had by implementing a multi-threaded approach, however, FileMaker Pro's import process has proven to be very efficient.
