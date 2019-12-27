use crate::frontend_url::FrontendUrl;
use crate::github_client_info::GitHubClientInfoFairing;
use crate::jwt_secret::JwtSecret;
use crate::main_db_conn::MainDbConn;
use crate::user_management::UserManagementMount;
use rocket::Rocket;

fn get_rocket() -> Rocket {
    rocket::ignite()
        .attach(MainDbConn::fairing())
        .attach(MainDbConn::migration_fairing())
        .attach(GitHubClientInfoFairing)
        .attach(FrontendUrl::fairing())
        .attach(JwtSecret::fairing())
        .mount_user_management()
}

pub fn start() {
    let r = get_rocket();

    info!("Starting rocket!");
    r.launch();
}
