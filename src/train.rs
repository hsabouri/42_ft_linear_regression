use tools::read_csv;
use std::process;
use clap;
use gnuplot::{Figure, Caption, Color};

use std::fmt;

struct Training {
    data: Vec<(f32, f32)>,
    min: f32,
    max: f32,
    theta_0: f32,
    theta_1: f32,
    learning_rate: f32,
    iterations: u32
}

impl Training {
    fn new(data: Vec<(f32, f32)>, learning_rate: f32, iterations: u32) -> Training {
        let min = {
            let mut min = data[0].0;

            for val in data.iter() {
                if val.0 < min { min = val.0 }
                if val.1 < min { min = val.1 }
            }
            min
        };
        let max = {
            let mut max = data[0].0;

            for val in data.iter() {
                if val.0 > max { max = val.0 }
                if val.1 > max { max = val.1 }
            }
            max
        };

        Training {
            data: data.iter().map(|val| {
                if min.abs() > max.abs() {
                    (val.0 / min.abs(), val.1 / min.abs())
                } else {
                    (val.0 / max.abs(), val.1 / max.abs())
                }
            }).collect(),
            min: min,
            max: max,
            theta_0: 0.0,
            theta_1: 0.0,
            learning_rate: learning_rate,
            iterations: iterations
        }
    }

    fn draw(&self) {
       let x = [0u32, 1, 2];
        let y = [3u32, 4, 5];
        let mut fg = Figure::new();

        fg.axes2d()
        .lines(&x, &y, &[Caption("A line"), Color("black")]);
        fg.show(); 
    }

    fn train(&self) -> (f32, f32) {
        let alpha = self.learning_rate;
        let m = self.data.len() as f32;
        let (mut theta_0, mut theta_1): (f32, f32) = (1.0, 1.0);

        for _ in 0..self.iterations {
            let (mut cost_0, mut cost_1): (f32, f32) = (0.0, 0.0);

            for i in self.data.iter() {
                let (x, y) = (i.0, i.1);

                cost_0 += (theta_1 * x + theta_0) - y;
                cost_1 += ((theta_1 * x + theta_0) - y) * x;
            }
            theta_0 -= alpha * 1.0 / m * cost_0;
            theta_1 -= alpha * 1.0 / m * cost_1;
        }

        (theta_0, theta_1)
    }
}

impl fmt::Display for Training {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "x,y")?;
        for value in self.data.iter() {
            writeln!(f, "{},{}", value.0, value.1)?;
        }
        write!(f, "")
    }
}

pub fn sub_training(matches: &clap::ArgMatches) {
    let filename: &str = matches.value_of("DATASET").unwrap();
    let data: Vec<(f32, f32)> = match read_csv(filename, 2) {
        Ok(data) => data,
        Err(err) => {
            println!("{}", err);
            process::exit(1);
        }
    };
    let learning_rate: f32 = matches.value_of("learning_rate")
                                    .unwrap()
                                    .parse::<f32>()
                                    .unwrap_or(0.001);
    let iterations: u32 = matches.value_of("iterations")
                                    .unwrap()
                                    .parse::<u32>()
                                    .unwrap_or(100);
    
    let training = Training::new(data, learning_rate, iterations);
    println!("{}", training);
    let (mut t0, mut t1) = training.train();
    //t1 = Training::remap(t1, 0.0, 1.0, training.min, training.max);
    t0 *= if training.min.abs() > training.max.abs() { training.min.abs() } else { training.max.abs() };
    println!("Thetas");
    eprintln!("{}\n{}", t0, t1);
    training.draw();
}
