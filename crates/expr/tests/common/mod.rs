use simple_logger::SimpleLogger;

pub fn setup_test_logger() {
    let result = SimpleLogger::new().init();
    if result.is_err() {
        eprintln!("Error initialising logger");
    }
}
