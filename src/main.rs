extern crate gnuplot;
use std::error::Error;
use crate::gnuplot::AxesCommon;
use gnuplot::{Figure, Caption, Color, DashType};

/*
 *starter values:
 *sigma = 10
 *beta = 8/3
 *rho = 28
 */


fn main() -> Result<(), Box<dyn Error>>{

    //our constants to create a lorenz attractor
    let sigma = 10.0;
    let beta = 8.0 / 3.0;
    let rho = 28.0;

    //working variables
    let mut x = 1.0;
    let mut y = 1.0;
    let mut z = 1.0;

    //value vectors
    let mut x_values = Vec::new();
    let mut y_values = Vec::new();
    let mut z_values = Vec::new();

    //step size
    let mut t = 0.01;

    //loop through and calculate the next state using the Lorenz equations
    while t < 10000.0 {
        let x = x + t * sigma * (y - x);
        let y = y + t * (x * (rho - z) - y);
        let z = z + t * (x * y - beta * z);
        t = t + 0.01;

        //add the values to the vectors
        x_values.push(x);
        y_values.push(y);
        z_values.push(z);
    }


    //draw a 3D plot with our x,y,z values
    let mut fg = Figure::new();
    fg.axes3d()
        .set_title("Lorenz Attractor", &[])
        .lines(x_values.iter(), y_values.iter(), z_values.iter(),
        &[Caption("Lorenz Attractor"), Color("red"), gnuplot::LineStyle(DashType::Solid)]);

    let result = fg.show();
    if result.is_err() {
        println!("Error: {}", result.err().unwrap());
    }

    Ok(())
}
