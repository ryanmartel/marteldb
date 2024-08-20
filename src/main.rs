use marteldb::repl::Repl;
use marteldb::storage::page;


fn main() -> Result<(), String> {
    let r = Repl::new();
    r.run()
}
