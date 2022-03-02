use large_squidlib::input_types::Row;
use large_squidlib::runner::Runner;


fn main() {
    // Get arguments
    let file_path = match std::env::args_os().nth(1) {
        Some(file_path) => file_path,
        None => panic!("expected 1 argument, but got none"),
    };
    // Open file
    let file = std::fs::File::open(file_path).unwrap();
    // Create a reader
    let mut reader = csv::Reader::from_reader(file);
    // Create a runner
    let mut runner = Runner::new();
    // Run file lines
    for result in reader.deserialize() {
        // Create row
        let record: Row = result.unwrap();
        // Run row
        runner.run_row(record);
    }
    // Create a writer
    let mut writer = csv::WriterBuilder::new()
        .has_headers(true)
        .flexible(true)
        .double_quote(false)
        .from_writer(std::io::stdout());
    // Create Output
    let output = runner.output();
    // Write Output
    for row in output {
        writer.serialize(row).unwrap()
    }
    writer.flush().unwrap();    
}
