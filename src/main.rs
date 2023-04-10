use secrecy::ExposeSecret;
use sqlx::postgres::PgPool;

use std::net::TcpListener;

use zero2prod::configuration::get_configuration;
use zero2prod::startup::run;
use zero2prod::telemetry;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let subsciber = telemetry::get_subscriber("zero2prod".into(), "info".into());
    telemetry::init_subscriber(subsciber);

    let configuration = get_configuration().expect("Failed to read configuration.");

    let connection_pool =
        PgPool::connect_lazy(configuration.database.connection_string().expose_secret())
            .expect("Failed to connect to Postgres.");

    let address = format!(
        "{}:{}",
        configuration.application.host, configuration.application.port
    );

    let listener = TcpListener::bind(address)?;

    run(listener, connection_pool)?.await?;

    Ok(())
}
