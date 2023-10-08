use actix_files as fs;
use actix_web::{get, post, web, App, Error, HttpResponse, HttpServer, Responder};
use serde::{Deserialize, Serialize};

use html_to_string_macro::html;

use crate::client::ui::Component;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Dashboard {}

impl Component for Dashboard {
    fn render(&self) -> String {
        let result = html!(
            <section class="section is-small p-4">
                <section class="columns">
                    <div class="column is-one-fifth">
                        <h1 class="title">"Hello actix webserver!"</h1>
                        <h2 class="subtitle">"actix-web + htmx + rusqlite = <3"</h2>
                    </div>

                    <div class="column">
                        <h1 class="title">"Hello actix webserver!"</h1>
                        <h2 class="subtitle">"actix-web + htmx + rusqlite = <3"</h2>
                    </div>

                    <div class="column is-one-quarter">
                        <h1 class="title">"Hello actix webserver!"</h1>
                        <h2 class="subtitle">"actix-web + htmx + rusqlite = <3"</h2>
                    </div>
                </section>
            </section>
        );

        result
    }
}

impl Dashboard {
    pub fn new() -> Self {
        Dashboard {}
    }
}
