use std::sync::Mutex;
use actix_web::web::{get, post, ServiceConfig, Data, Form};
use actix_web::{HttpResponse, Responder};
use serde::Deserialize;
use crate::state::{AppState, Entry};

#[derive(Deserialize)]
struct EntryFormData {
    author: String,
    comment: String,
}

fn render_entry(entry: &Entry) -> String {
    format!("<strong>{}</strong><br />{}", entry.author, entry.comment)
}

async fn render_index(data: Data<Mutex<AppState>>) -> impl Responder {
    let header = "<html><body>";
    let footer = "</body></html>";
    let title = "<h1>Guestbook</h1>";
    let form = r#"
        <form action="/" method="POST">
            <input name="author" type="text" placeholder="Your name" /><br />
            <textarea name="comment">
            </textarea><br />
            <input type="submit" value="Submit">
        </form>
    "#;

    let state = data.lock().unwrap();
    let entries = state.entries.iter().map(|e| render_entry(&e)).collect::<Vec<_>>();

    let response = format!("{}{}{}<hr />{}{}", header, title, form, entries.join("<br />"), footer);

    HttpResponse::Ok()
        .content_type("text/html")
        .body(response)
}

async fn post_entry(data: Data<Mutex<AppState>>, input: Form<EntryFormData>) -> impl Responder {
    let new_entry = Entry { author: input.author.clone(), comment: input.comment.clone() };
    {
        let mut state = data.lock().unwrap();
        state.entries.push(new_entry);
    }

    render_index(data).await
}

pub fn routes(config: &mut ServiceConfig) {
    config
        .route("", get().to(render_index))
        .route("", post().to(post_entry));
}