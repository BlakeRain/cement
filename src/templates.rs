use lazy_static::lazy_static;
use minijinja::{Environment, path_loader};

lazy_static! {
    pub static ref TEMPLATES: Environment<'static> = {
        let mut env = Environment::new();
        env.set_loader(path_loader("templates"));
        env
    };
}
