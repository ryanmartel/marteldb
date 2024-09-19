use marteldb::repl;

fn main() -> Result<(), String> {
    let r = Repl::new();
    r.run()
}
