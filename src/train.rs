use tools::read_csv;
use std::process;
use clap;
use gnuplot::{Figure, Caption, Color};

use std::fmt;

struct Training {
    data: Vec<(f32, f32)>,
    min: f32,
    max: f32,
    x_max: f32,
    x_min: f32,
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
        let x_min = {
            let mut min = data[0].0;

            for val in data.iter() {
                if val.0 < min { min = val.0 }
            }
            min
        };
        let x_max = {
            let mut max = data[0].0;

            for val in data.iter() {
                if val.0 > max { max = val.0 }
            }
            max
        };

        Training {
            data: data,
            min: min,
            max: max,
            x_min: x_min,
            x_max: x_max,
            theta_0: 0.0,
            theta_1: 0.0,
            learning_rate: learning_rate,
            iterations: iterations
        }
    }

    fn scale(&mut self) {
        self.data = self.data.iter().map(|val| {
            if self.min.abs() > self.max.abs() {
                (val.0 / self.min.abs(), val.1 / self.min.abs())
            } else {
                (val.0 / self.max.abs(), val.1 / self.max.abs())
            }
        }).collect();
    }
    
    fn unscale(&self) -> Vec<(f32, f32)> {
        self.data.iter().map(|val| {
            if self.min.abs() > self.max.abs() {
                (val.0 * self.min.abs(), val.1 * self.min.abs())
            } else {
                (val.0 * self.max.abs(), val.1 * self.max.abs())
            }
        }).collect()
    }

    fn draw(&self) {
        let mut fg = Figure::new();
        let (x, y): (Vec<f32>, Vec<f32>) = self.unscale().iter().cloned().unzip();
        let (px, py): (Vec<f32>, Vec<f32>) = (vec![self.x_min, self.x_max],
                                              vec![self.x_min * self.theta_1 + self.theta_0,
                                                   self.x_max * self.theta_1 + self.theta_0]);

        fg.axes2d()
        .lines(&x, &y, &[Caption("Dataset"), Color("black")])
        .lines(&px, &py, &[Caption("Prediction"), Color("red")]);
        fg.show(); 
    }

    fn train(&mut self) -> (f32, f32) {
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
    
    let mut training = Training::new(data, learning_rate, iterations);
    training.scale();
    let (t0, t1) = training.train();
    training.theta_0 = t0 * if training.min.abs() > training.max.abs() { training.min.abs() } else { training.max.abs() };
    training.theta_1 = t1;
    println!("Thetas");
    println!("{}\n{}", training.theta_0, training.theta_1);
    training.draw();
}
