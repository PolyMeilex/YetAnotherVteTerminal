use gio::prelude::*;

mod app;
mod state;

fn main() {
    let app = gtk::ApplicationBuilder::new()
        .application_id("io.github.polymeilex.yavt")
        .flags(gio::ApplicationFlags::FLAGS_NONE)
        .build();

    gtk::init().unwrap();

    app.connect_activate(|app| {
        let mut app = app::App::new(app);
        app.connect();
    });

    app.run(&[]);
}
