mod day01;
mod day02;

fn run_fn<T>(f: fn() -> T) {
    f();
}

fn main() {
    let mut args = std::env::args();
    if args.len() == 1 {
        println!("Usage: {} ddpp (where dd is day number and pp is problem number)", args.next().unwrap());
        return;
    }
    args.for_each(|arg| {
        match arg.as_str() {
            "0101" => run_fn(day01::soln01),
            "0102" => run_fn(day01::soln02),
            "0201" => run_fn(day02::soln01),
            "0202" => run_fn(day02::soln02),
            _ => println!("Unknown argument: {}", arg),
        }
    });
}
