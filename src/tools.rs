extern crate csv;

pub fn read_csv(path: &str, cols: usize) -> Result<Vec<(f32, f32)>, &'static str> {
    let mut rdr = match csv::Reader::from_path(path) {
        Ok(rdr) => rdr,
        Err(_) => return Err("Error while reading CSV file")
    };
    let mut res: Vec<(f32, f32)> = Vec::new();

    for line in rdr.records() {
        let record = match line {
            Ok(record) => record,
            Err(_) => return Err("Error while reading CSV file")
        };

        if record.len() != cols { return Err("CSV file is not a valid dataset") }
 
        res.push(match ( // Parsing, checking and pushing numbers
            record[0].parse::<f32>(),
            record[1].parse::<f32>()
        ) {
            (Ok(x), Ok(y)) => (x, y),
            _ => return Err("Not a number")
        });
    }
    Ok(res)
}

pub fn read_csv_dimensions(path: &str, cols: usize) -> Result<Vec<Vec<f32>>, &'static str> {
    let mut rdr = match csv::Reader::from_path(path) {
        Ok(rdr) => rdr,
        Err(_) => return Err("Error while reading CSV file")
    };
    let mut res: Vec<Vec<f32>> = Vec::new();

    for line in rdr.records() {
        let record = match line {
            Ok(record) => record,
            Err(_) => return Err("Error while reading CSV file")
        };

        if record.len() != cols { return Err("CSV file dimension is not valid") }
 
        let to_push: Vec<f32> = record.into_iter()
                            .map(|value| { value.parse::<f32>().ok() })
                            .fuse()
                            .filter_map(|x| x)
                            .collect();
        //Add line number for errors
        if to_push.len() != record.len() {
            return Err("Not a number")
        }
        res.push(to_push);
    }
    Ok(res)
}
