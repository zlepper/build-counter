use crate::api::user::UserApiMount;
use crate::frontend_url::{AttachFrontendUrlState, FrontendUrl};
use crate::github_client_info::GitHubClientInfoFairing;
use crate::jwt_secret::JwtSecret;
use crate::main_db_conn::MainDbConn;
use crate::user_management::UserManagementMount;
use rocket::{fairing, Rocket};
use rocket_cors::AllowedOrigins;

trait CorsFairing {
    fn add_cors(self) -> Self;
}

impl CorsFairing for Rocket {
    fn add_cors(self) -> Self {
        let frontend_url = self
            .state::<FrontendUrl>()
            .expect("Frontend url was not attached");

        let allowed_origins = AllowedOrigins::some_exact(&[(*frontend_url).to_string()]);

        let cors = rocket_cors::CorsOptions {
            allowed_origins,
            ..Default::default()
        }
        .to_cors()
        .expect("Failed to create cors configuration");

        self.attach(cors)
    }
}

fn get_rocket() -> Rocket {
    rocket::ignite()
        .attach(MainDbConn::fairing())
        .attach(MainDbConn::migration_fairing())
        .attach(GitHubClientInfoFairing)
        .attach_frontend_url_state()
        .attach(JwtSecret::fairing())
        .add_cors()
        .mount_user_management()
        .mount_user_api()
}

pub fn start() {
    let r = get_rocket();

    info!("Starting rocket!");
    r.launch();
}
