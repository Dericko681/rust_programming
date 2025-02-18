use application::Application;

fn main() {
   let mut app = Application::new();
   app.run();
}
mod models;
mod data_c;
mod enums;
mod application;
mod traits;