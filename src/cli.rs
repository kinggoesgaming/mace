/// Mace compiler
#[derive(clap::Parser, Debug)]
#[command(about, author, version)]
pub struct Cli {
    #[clap(subcommand)]
    sub_command: Option<Command>,
}

#[derive(clap::Subcommand, clap::ValueEnum, Clone, Debug)]
pub enum Command {
    /// Add a dependency
    Add,
    /// Build the Mace project
    Build,
    /// Generate docs for the Mace project
    Docs,
    /// Install the Mace project to a Minecraft game
    Install,
    /// Create a new Mace project
    New,
    /// Remove a dependency
    Remove,
    /// Test the Mace project
    Test,
}
