use num::complex::Complex;

fn main() {
    let height = 30;
    let width = 150;
    let max_iters = 1000;
    let y_min = -1.5;
    let y_max = 1.5;
    let x_min = -1.5;
    let x_max = 0.5;
    render_mandelbrot(create_escape_vals(
        height, width, max_iters, y_min, y_max, x_min, x_max,
    ));
}

fn create_escape_vals(
    height: usize,
    width: usize,
    max_iters: u32,
    y_min: f64,
    y_max: f64,
    x_min: f64,
    x_max: f64,
) -> Vec<Vec<u32>> {
    let mut rows: Vec<Vec<u32>> = Vec::with_capacity(height);
    for y in 0..height {
        let mut row: Vec<u32> = Vec::with_capacity(width);
        for x in 0..width {
            let c = Complex {
                re: ((x as f64) / (width as f64)) * (x_max - x_min) + x_min,
                im: ((y as f64) / (height as f64)) * (y_max - y_min) + y_min,
            };
            let escape_val = calc_escape_val(c, max_iters);
            row.push(escape_val);
        }
        rows.push(row);
    }
    rows
}

fn calc_escape_val(c: Complex<f64>, max_iters: u32) -> u32 {
    let mut z = Complex { re: 0.0, im: 0.0 };
    for i in 0..=max_iters {
        if z.norm() > 10.0 {
            return i;
        }
        z = z.powi(2) + c;
    }
    max_iters
}

fn render_mandelbrot(escape_vals: Vec<Vec<u32>>) {
    for row in escape_vals {
        let mut line = String::with_capacity(row.len());
        for point in row {
            let render_char = match point {
                0..=2 => ' ',
                2..=5 => '.',
                5..=10 => '•',
                11..=30 => '*',
                30..=100 => '+',
                100..=200 => 'x',
                200..=400 => '$',
                400..=700 => '#',
                _ => '%',
            };
            line.push(render_char);
        }
        println!("{}", line);
    }
}
