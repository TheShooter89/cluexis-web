use std::collections::HashMap;

use actix_files as fs;
use actix_web::{get, post, web, App, Error, HttpResponse, HttpServer, Responder};
use serde::{Deserialize, Serialize};

use html_to_string_macro::html;

use crate::client::ui::{
    ChatPanel, Component, StatisticsBox, StatisticsBoxData, StatisticsBoxDataGroup, TitleStrip,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Dashboard {}

impl Component for Dashboard {
    fn render(&self) -> String {
        let mut stat_box = StatisticsBoxDataGroup {
            name: "Aggregate".to_string(),
            data: Vec::new(),
        };
        stat_box.data.push(StatisticsBoxData {
            label: "Total days:".to_string(),
            value: "217".to_string(),
        });
        stat_box.data.push(StatisticsBoxData {
            label: "Total messages (raw):".to_string(),
            value: "18673(16736)".to_string(),
        });
        stat_box.data.push(StatisticsBoxData {
            label: "From me:".to_string(),
            value: "6718(5972)".to_string(),
        });
        println!("stat_box: {:?}", stat_box);

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
                        {StatisticsBox::new().group(stat_box).render()}
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
