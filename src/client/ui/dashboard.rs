use actix_files as fs;
use actix_web::{get, post, web, App, Error, HttpResponse, HttpServer, Responder};
use serde::{Deserialize, Serialize};

use html_to_string_macro::html;

use crate::client::ui::{ChatPanel, Component, TitleStrip};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Dashboard {}

impl Component for Dashboard {
    fn render(&self) -> String {
        let result = html!(
            <section class="section p-4 cw-dashboard-container">
                <section class="columns">
                    <div class="column is-one-fifth">
                        {ChatPanel::new().render()}
                    </div>

                    <div class="column">
                        {TitleStrip::new().add("Andrea").add("Carla").render()}
                    </div>

                    <div class="column is-one-quarter">
                        {TitleStrip::new().add("statistics").render()}
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
