use cli::repl::Repl;

fn main() -> Result<(), String> {
    let r = Repl::new();
    r.run()
}
