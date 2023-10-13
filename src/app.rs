use poem::{
    error::InternalServerError,
    get, handler,
    http::{header::LOCATION, HeaderMap, HeaderValue, StatusCode},
    web::{Data, Form, Html, Path, RemoteAddr},
    EndpointExt, IntoEndpoint, Route,
};
use serde::Deserialize;
use tera::Context;

use crate::{env::Env, model::Post, templates::TEMPLATES};

fn default_context() -> Context {
    let mut context = Context::new();

    context.insert("version", env!("CARGO_PKG_VERSION"));
    context.insert("build_info", env!("CARGO_BUILD_INFO"));

    context
}

#[handler]
fn handle_index_get() -> poem::Result<Html<String>> {
    let context = default_context();
    TEMPLATES
        .render("index.html", &context)
        .map_err(InternalServerError)
        .map(Html)
}

#[derive(Deserialize)]
struct CreatePaste {
    content: String,
}

#[handler]
async fn handle_index_post(
    env: Data<&Env>,
    remote: &RemoteAddr,
    Form(CreatePaste { content }): Form<CreatePaste>,
) -> poem::Result<(StatusCode, HeaderMap, ())> {
    let slug = Post::create(&env.pool, remote, content)
        .await
        .map_err(InternalServerError)?;

    Ok((
        StatusCode::FOUND,
        HeaderMap::from_iter([(
            LOCATION,
            HeaderValue::from_str(&format!("/{}", slug)).expect("header value"),
        )]),
        (),
    ))
}

#[handler]
async fn handle_paste_get(env: Data<&Env>, Path(code): Path<String>) -> poem::Result<Html<String>> {
    let post = Post::get(&env.pool, &code)
        .await
        .map_err(InternalServerError)?;

    let mut context = default_context();
    context.insert("post", &post);
    TEMPLATES
        .render("paste.html", &context)
        .map_err(InternalServerError)
        .map(Html)
}

pub fn create_app(env: Env) -> impl IntoEndpoint {
    Route::new()
        .at("/", get(handle_index_get).post(handle_index_post))
        .at("/:code", get(handle_paste_get))
        .data(env)
}
