// define the mod in the parent module
mod config;
mod models;
mod routes;

fn main() {
    println!("Hello, main!");
    config::print_config();
    routes::health_route::print_health_route();
    routes::user_route::print_user_route();
    models::user_model::print_user_model();
}
