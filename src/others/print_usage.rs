/// Prints the usage instructions for the program.
pub fn print_usage() {
    eprintln!("Usage: fileforge <command> [--ignore-git]");
    eprintln!("Commands:");
    eprintln!("  init      Generate configuration");
    eprintln!("  generate  Generate the Dockerfile");
    eprintln!("  config    Print the current configuration");
    eprintln!("  version   Print the version of fileforge");
}
