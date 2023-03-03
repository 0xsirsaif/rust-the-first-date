use num::complex::Complex;

mod safe_rust;


fn main() {
    list_of_langs();
    print!(">>>>>>>>>\n");
    basic_text_processing();
    print!(">>>>>>>>>\n");
    add_nums();
    print!(">>>>>>>>>\n");
    base_x();
    print!(">>>>>>>>>\n");
    basic_loop();
    print!(">>>>>>>>>\n");
    ternary_operator();
    print!(">>>>>>>>>\n");
    wtf_loop_1();
    print!(">>>>>>>>>\n");
    wtf_loop_2();
    print!(">>>>>>>>>\n");
    wtf_loop_3();
    print!(">>>>>>>>>\n");
    let mandelbrot = calculate_mandelbrot(1000, -2.0, 1.0, -1.0, 1.0, 100, 24);
    render_mandelbrot(mandelbrot);
    print!(">>>>>>>>>\n");
}


fn is_even(n: i8) -> bool {
    n % 2 == 0
}


fn ternary_operator() {
    let n: i8 = 10;
    let description = if is_even(n) {
        "even"
    } else {
        "odd"
    };
    println!("{} is {}", n, description);
}


fn basic_text_processing() {
    let raw_data = "\
    common name,length (cm)
    Little penguin,33
    Yellow-eyed penguin,65
    Fiordland penguin,60
    Invalid,data
    ";

    let records = raw_data.lines();

    for (i, record) in records.enumerate() {
        // skip the header and empty lines.
        if i == 0 || record.trim().is_empty() {
            continue;
        }

        // split the record into fields.
        let fields: Vec<_> = record
            .split(',')
            .map(|field| field.trim())
            .collect();

        // cfg! is a macro that returns true if the given configuration is set.
        if cfg!(debug_assertions) {
            eprint!("debug: {:?} -> {:?}\n", record, fields)
        }

        let name = fields[0];
        // parse the length field as a float. ::<f32> is a type annotation.
        if let Ok(length) = fields[1].parse::<f32>() {
            println!("{}, {}cm\n", name, length)
        }
    }
}


fn list_of_langs() {
    let arabic = "محمد سيف";
    let english = "Muhammad Saif";
    let list_of_lang = [arabic, english];
    for lang in list_of_lang {
        // the ampersand is borrows lang for read only access.
        println!("{}", &lang);
    }
}


fn add_nums() {
    // there are multiple syntactic choices for annotating data types to integers:

    // Types can be inferred by the compiler…
    let a = 10;
    // …or declared by the programmer when creating variables.
    let b: i32 = 20;
    // Numeric types can include a type annotation in their literal form.
    let c = 30i32;
    // Numbers can include underscores, which are intended to increase readability and have no functional impact.
    let d = 40_i32;
    let e = add(add(a, b), add(c, d));

    println!("(a + b) + (c + d) = {}", e);
}


fn add(a: i32, b: i32) -> i32 {
    // return a + b, Functions return the last expression’s result so that return is not required.
    // a + b; if ; added? __ This changes the semantics, returning () (unit) rather than i32.
    a + b
}


fn base_x() {
    // base 2
    let three = 0b11;
    // base 8
    let thirty = 0o36;
    // base 16
    let three_hundred = 0x12C;
    println!("base 10: {}, {}, {}", three, thirty, three_hundred);
    println!("base 2: {:b}, {:b}, {:b}", three, thirty, three_hundred);
    println!("base 8: {:o}, {:o}, {:o}", three, thirty, three_hundred);
    println!("base 16: {:x}, {:x}, {:x}", three, thirty, three_hundred);
}


fn basic_loop() {
    for i in 0..10 {
        println!("{}", i)
    }
    print!(">>\n");
    for i in 0..=10 {
        println!("{}", i)
    }
}


fn wtf_loop_1() {
    let container = [1, 2, 3, 4, 5];
    for i in container {
        println!("I: {}", i);
    }
    println!("container: {:?}", container);
}


fn wtf_loop_2() {
    let container = [1, 2, 3, 4, 5];
    for i in &container {
        println!("I2: {}", i);
    }
    println!("container: {:?}", container);
}


fn wtf_loop_3() {
    let mut container = [1, 2, 3, 4, 5];
    for i in &mut container {
        // dereference i and multiply it by 10.
        *i *= 10;
        println!("I: {}", i);
    }
    println!("container: {:?}", &container);
}


fn calculate_mandelbrot(      // <2>

                              max_iters: usize,           // <3>
                              x_min: f64,                 // <4>
                              x_max: f64,                 // <4>
                              y_min: f64,                 // <4>
                              y_max: f64,                 // <4>
                              width: usize,               // <5>
                              height: usize,              // <5>
) -> Vec<Vec<usize>> {
    let mut rows: Vec<_> = Vec::with_capacity(width); // <6>
    for img_y in 0..height {                          // <7>

        let mut row: Vec<usize> = Vec::with_capacity(height);
        for img_x in 0..width {
            let x_percent = (img_x as f64 / width as f64);
            let y_percent = (img_y as f64 / height as f64);
            let cx = x_min + (x_max - x_min) * x_percent; // <8>
            let cy = y_min + (y_max - y_min) * y_percent; // <8>
            let escaped_at = mandelbrot_at_point(cx, cy, max_iters);
            row.push(escaped_at);
        }

        rows.push(row);
    }
    rows
}


fn mandelbrot_at_point(   // <9>
                          cx: f64,
                          cy: f64,
                          max_iters: usize,
) -> usize {
    let mut z = Complex { re: 0.0, im: 0.0 };       // <10>
    let c = Complex::new(cx, cy);                   // <11>

    for i in 0..=max_iters {
        if z.norm() > 2.0 {                           // <12>
            return i;
        }
        z = z * z + c;                                // <13>
    }
    max_iters                                       // <14>
}


fn render_mandelbrot(escape_vals: Vec<Vec<usize>>) {
    for row in escape_vals {
        let mut line = String::with_capacity(row.len());
        for column in row {
            let val = match column {
                0..=2 => ' ',
                3..=5 => '.',
                6..=10 => '•',
                11..=30 => '*',
                31..=100 => '+',
                101..=200 => 'x',
                201..=400 => '$',
                401..=700 => '#',
                _ => '%',
            };

            line.push(val);
        }
        println!("{}", line);
    }
}