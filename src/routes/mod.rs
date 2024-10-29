use actix_web::web;

mod vehicle;
mod user;

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(vehicle::get_vehicle);
    cfg.service(user::register_user);
}
