extern crate gnuplot;
use std::io;
use crate::gnuplot::AxesCommon;
use gnuplot::{Figure, Caption, Color, DashType};

fn main() -> io::Result<()> {

    //our constants that generate the chaotic behaviour we want
    let sigma = 10.0;
    let beta = 8.0 / 3.0;
    let rho = 28.0;

    //initial conditions
    let x0 = 1.0;
    let y0 = 1.0;
    let z0 = 1.0;

    //variables used to solve the Lorenz system
    let dt = 0.01;
    let mut t = 0.0;
    let mut x = x0;
    let mut y = y0;
    let mut z = z0;

    //value vectors
    let mut x_values = vec![x0];
    let mut y_values = vec![y0];
    let mut z_values = vec![z0];

    //get user input to determine iterations
    let mut input = String::new();
    println!("Input the number of iterations to pass through: ");
    io::stdin().read_line(&mut input)?;

    let iterations = match input.trim().parse::<i32>() {
        Ok(n) => n,
        Err(e) => {
            println!("Error: {}", e);
            return Ok(());
        }
    };

    
    //loop through and calculate the next state using the Lorenz equations
    while t < iterations as f64 {
        let dx = sigma * (y - x);
        let dy = x * (rho - z) - y;
        let dz = x * y - beta * z;

        x += dx * dt;
        y += dy * dt;
        z += dz * dt;

        t += dt;

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
