use rocket::fairing::{Fairing, Info, Kind};
use rocket::Rocket;

use crate::db::{run_migrations, Connection};

pub struct ConnectionInit();

impl ConnectionInit {
    pub fn fairing() -> Self {
        Self()
    }
}

#[rocket::async_trait]
impl Fairing for ConnectionInit {
    fn info(&self) -> Info {
        Info {
            name: "Initialize database connection",
            kind: Kind::Attach,
        }
    }

    async fn on_attach(&self, rocket: Rocket) -> Result<Rocket, Rocket> {
        match Connection::get_one(&rocket).await {
            Some(c) => match c.run(|x| run_migrations(&x)).await {
                Ok(_) => Ok(rocket),
                Err(_) => Err(rocket),
            },
            None => Ok(rocket),
        }
    }
}
