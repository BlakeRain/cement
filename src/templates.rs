use lazy_static::lazy_static;
use tera::Tera;

lazy_static! {
    pub static ref TEMPLATES: Tera = {
        let mut tera = Tera::new("templates/**/*").expect("to load templates");
        tera.autoescape_on(vec![".html"]);
        tera
    };
}
