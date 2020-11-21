// use gio::prelude::*;

mod app;
mod config;
// mod state;

fn main() {
    // let app = gtk::ApplicationBuilder::new()
    //     .application_id("io.github.polymeilex.yavt")
    //     .flags(gio::ApplicationFlags::FLAGS_NONE)
    //     .build();

    gtk::init().unwrap();

    let config = config::get_config();

    // app.connect_activate(move |app| {
    let mut app = app::App::new(&config);
    app.connect();
    // });

    gtk::main();

    // app.run(&[]);
}
