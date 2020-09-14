use gio::prelude::*;

mod app;
mod config;
mod state;

fn main() {
    let app = gtk::ApplicationBuilder::new()
        .application_id("io.github.polymeilex.yavt")
        .flags(gio::ApplicationFlags::FLAGS_NONE)
        .build();

    gtk::init().unwrap();

    app.connect_activate(|app| {
        let config = config::get_config();

        let mut app = app::App::new(app, config);
        app.connect();
    });

    app.run(&[]);
}
