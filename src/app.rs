use anyhow::Context;
use poem::{
    EndpointExt, IntoEndpoint, Route,
    endpoint::StaticFilesEndpoint,
    error::InternalServerError,
    get, handler,
    http::{HeaderMap, HeaderValue, StatusCode, header::LOCATION},
    middleware::Csrf,
    web::{CsrfToken, CsrfVerifier, Data, Form, Html, Path, RealIp},
};
use serde::{Deserialize, Serialize};

use crate::{env::Env, model::Post, templates::TEMPLATES};

fn default_context() -> minijinja::Value {
    minijinja::context! {
        version => env!("CARGO_PKG_VERSION"),
        build_date => env!("CARGO_BUILD_DATE"),
    }
}

fn render_template<S: Serialize>(name: &str, context: S) -> poem::Result<Html<String>> {
    let template = TEMPLATES
        .get_template(name)
        .context("failed to get template")?;
    let value = template
        .render(context)
        .context("failed to render template")?;
    Ok(Html(value))
}

#[handler]
fn handle_index_get(token: &CsrfToken) -> poem::Result<Html<String>> {
    render_template(
        "index.html",
        minijinja::context! {
            token => token.0,
            ..default_context()
        },
    )
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

    let slug = Post::create(&env.db, &remote, content, highlight)
        .await
        .context("failed to create post")?;

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
    let post = Post::get(&env.db, &code)
        .await
        .context("failed to get post")?;

    render_template(
        "paste.html",
        minijinja::context! {
            post,
            ..default_context()
        },
    )
}

pub fn create_app(env: Env) -> impl IntoEndpoint {
    Route::new()
        .at("/", get(handle_index_get).post(handle_index_post))
        .at("/:code", get(handle_paste_get))
        .nest("/static", StaticFilesEndpoint::new("static"))
        .data(env)
        .with(Csrf::new())
}
