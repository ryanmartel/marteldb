use marteldb::repl::Repl;
use marteldb::storage::page;


fn main() -> Result<(), String> {
    page::test_buffer();
    let r = Repl::new();
    r.run()
}
