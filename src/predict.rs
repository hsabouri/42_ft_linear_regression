use tools::read_csv_dimensions;
use std::fmt;
use clap;

struct Predict {
    theta: Vec<f32>,
}

impl Predict {
    fn new(theta: Vec<f32>) -> Predict {
        Predict {
            theta: theta
        }
    }

    fn predict(&self, dataset: &Vec<f32>) -> Vec<f32> {
        dataset.iter()
        .map(|x| {
            let mut res: f32 = 0.0;

            for (power, theta) in self.theta.iter().enumerate() {
                res += x.powi(power as i32) * theta;
            }
            res
        })
        .collect()
    }
}

pub fn sub_predict(matches: &clap::ArgMatches) -> Result<(), &'static str> {
    let filename: &str = matches.value_of("CONFIG_FILE").unwrap();
    let theta: Vec<f32> = read_csv_dimensions(filename, 1)?
        .iter()
        .map(|value| {
            value[0]
        }).collect();
    let data: Vec<f32> = match matches.value_of("DATASET") {
        Some (filename) => {
            read_csv_dimensions(filename, 1)?
            .iter()
            .map(|value| {
                value[0]
            }).collect()
        }
        None => match matches.value_of("X").unwrap().parse::<f32>() {
            Ok(value) => vec![value],
            Err(_) => return Err("Provided parameter X is invalid")
        }
    };
    
    eprintln!("{:?}", theta);

    let predictor = Predict::new(theta);
    let prediction = predictor.predict(&data);
    println!("x,y");
    for (x, y) in data.iter().zip(prediction.iter()) {
        println!("{},{}", x, y);
    }
    Ok(())
}
