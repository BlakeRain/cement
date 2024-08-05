use poem::{
    endpoint::StaticFilesEndpoint,
    error::InternalServerError,
    get, handler,
    http::{header::LOCATION, HeaderMap, HeaderValue, StatusCode},
    middleware::Csrf,
    web::{CsrfToken, CsrfVerifier, Data, Form, Html, Path, RealIp},
    EndpointExt, IntoEndpoint, Route,
};
use serde::Deserialize;
use tera::Context;

use crate::{env::Env, model::Post, templates::TEMPLATES};

fn default_context() -> Context {
    let mut context = Context::new();

    context.insert("version", env!("CARGO_PKG_VERSION"));
    context.insert("build_date", env!("CARGO_BUILD_DATE"));

    context
}

fn render_template(name: &str, context: &Context) -> poem::Result<Html<String>> {
    TEMPLATES
        .render(name, context)
        .map_err(|err| {
            tracing::error!("render '{name}' failed: {err:?}");
            InternalServerError(err)
        })
        .map(Html)
}

#[handler]
fn handle_index_get(token: &CsrfToken) -> poem::Result<Html<String>> {
    let mut context = default_context();
    context.insert("token", &token.0);
    render_template("index.html", &context)
}

#[derive(Deserialize)]
struct CreatePaste {
    token: String,
    highlight: String,
    content: String,
}

#[handler]
async fn handle_index_post(
    env: Data<&Env>,
    verifier: &CsrfVerifier,
    remote: RealIp,
    Form(CreatePaste {
        token,
        highlight,
        content,
    }): Form<CreatePaste>,
) -> poem::Result<(StatusCode, HeaderMap, ())> {
    if !verifier.is_valid(&token) {
        tracing::error!("CSRF token was invalid");

        return Ok((
            StatusCode::BAD_REQUEST,
            HeaderMap::from_iter([(LOCATION, HeaderValue::from_str("/").expect("header value"))]),
            (),
        ));
    }

    let highlight = if highlight.is_empty() {
        None
    } else {
        Some(highlight)
    };

    let slug = Post::create(&env.pool, &remote, content, highlight)
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
    render_template("paste.html", &context)
}

pub fn create_app(env: Env) -> impl IntoEndpoint {
    Route::new()
        .at("/", get(handle_index_get).post(handle_index_post))
        .at("/:code", get(handle_paste_get))
        .nest("/static", StaticFilesEndpoint::new("static"))
        .data(env)
        .with(Csrf::new())
}
