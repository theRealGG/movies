use movies::telematry::{init_subscriber, subscriber};

#[tokio::main]
async fn main() {
    init_subscriber(subscriber("movies".into(), std::io::stdout));
}
