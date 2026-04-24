use std::fs::File;
use std::io::BufReader;
use std::sync::Mutex;

static mut PRICE_READER: Option<Mutex<csv::StringRecordsIntoIter<BufReader<File>>>> = None;

pub fn fetch_price() -> Option<f32> {
    use std::sync::Once;
    static INIT: Once = Once::new();

    INIT.call_once(|| {
        let file = File::open("data/SP.csv").expect("Failed to open data/SP.csv");
        let reader = BufReader::new(file);
        let mut csv_reader = csv::Reader::from_reader(reader);
        // Skip header if present
        let _ = csv_reader.headers();
        let iter = csv_reader.into_records();
        unsafe {
            PRICE_READER = Some(Mutex::new(iter));
        }
    });

    unsafe {
        if let Some(ref mutex) = PRICE_READER {
            let mut iter = mutex.lock().unwrap();
            if let Some(Ok(record)) = iter.next() {
                // Assume price is in the first column
                if let Some(price_str) = record.get(2) {
                    match price_str.parse::<f32>() {
                        Ok(price) => return Some(price),
                        Err(e) => {
                            eprintln!("[DEBUG] Failed to parse price from '{}': {}", price_str, e);
                        }
                    }
                } else {
                    eprintln!("[DEBUG] No value in price column: {:?}", record);
                }
            } else {
                // Rewind to start if at end
                let file = File::open("data/SP.csv").expect("Failed to open data/SP.csv");
                let reader = BufReader::new(file);
                let mut csv_reader = csv::Reader::from_reader(reader);
                let _ = csv_reader.headers();
                let mut new_iter = csv_reader.into_records();
                if let Some(Ok(record)) = new_iter.next() {
                    if let Some(price_str) = record.get(2) {
                        match price_str.parse::<f32>() {
                            Ok(price) => {
                                *iter = new_iter;
                                return Some(price);
                            },
                            Err(e) => {
                                eprintln!("[DEBUG] Failed to parse price from '{}': {}", price_str, e);
                            }
                        }
                    } else {
                        eprintln!("[DEBUG] No value in price column: {:?}", record);
                    }
                }
                *iter = new_iter;
            }
        }
    }
    None
}