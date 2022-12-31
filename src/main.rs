use clap::Parser;
//mod pointer_examples;

#[derive(Parser)]
#[derive(Debug)] //that is just for debug printing the struct
struct Cli {
    pattern: String,
    path: std::path::PathBuf,
}

fn main() {
    //Uncomment and test out the different errors Rust will throw at you!
    //pointer_examples::double_free();
    //pointer_examples::dereferencing_freed_memory();
    //pointer_examples::borrowing();
    let args = Cli::parse();
    let content = std::fs::read_to_string(&args.path).expect("could not read file");
    for line in content.lines() {
        if line.contains(&args.pattern) {
            println!("{}", line);
        }
    }
}