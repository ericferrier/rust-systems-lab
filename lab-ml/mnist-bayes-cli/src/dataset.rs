use std::error::Error;

pub fn load_csv(path: &str) -> Result<Vec<(Vec<f64>, usize)>, Box<dyn Error>> {
    let mut rdr = csv::Reader::from_path(path)?;
    let mut data = Vec::new();

    for result in rdr.records() {
        let record = result?;
        let label: usize = record[0].parse()?;

        let features: Vec<f64> = record.iter()
            .skip(1)
            .map(|v| v.parse::<f64>().unwrap() / 255.0)
            .collect();

        data.push((features, label));
    }

    Ok(data)
}