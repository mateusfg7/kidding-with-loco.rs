use kidding_with_loco::app::App;
use loco_rs::cli;
use migration::Migrator;

#[tokio::main]
async fn main() -> eyre::Result<()> {
    cli::main::<App, Migrator>().await
}
