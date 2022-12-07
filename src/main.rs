mod day01;
mod day02;

fn main() {
    let args = std::env::args();
    if args.len() == 1 {
        println!("Usage: cargo run -- ddpp (where dd is day number and pp is problem number)");
        return;
    }
    args.skip(1).for_each(|arg| {
        match arg.as_str() {
            "0101" => day01::soln01(),
            "0102" => day01::soln02(),
            "0201" => day02::soln01(),
            "0202" => day02::soln02(),
            _ => println!("Unknown argument: {}", arg),
        }
    });
}
