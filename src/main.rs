use log::info;
use tokio;
mod tools;
mod task1;
mod task2;
mod task3;

#[tokio::main]
async fn main() {
    tools::load_and_check_env();
    task1::do_task1().await;
    task2::do_task2().await;
    task3::do_task3().await;
    info!("Done");
}