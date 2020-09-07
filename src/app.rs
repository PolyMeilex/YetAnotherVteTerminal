use gtk::prelude::*;

use crate::state::State;
use std::{cell::RefCell, rc::Rc};

use vte::TerminalExt;
use vte::TerminalExtManual;

const LINK_EXPR: &str = "(((file|http|ftp|https)://)|(www|ftp)[-A-Za-z0-9]*\\.)[-A-Za-z0-9\\.]+(:[0-9]*)?(/[-A-Za-z0-9_\\$\\.\\+\\!\\*\\(\\),;:@&=\\?/~\\#\\%]*[^]'\\.}>\\) ,\\\"])?";

pub struct App {
    vte: vte::Terminal,
    win: gtk::ApplicationWindow,
    state: Rc<RefCell<State>>,
}

impl App {
    pub fn new(app: &gtk::Application) -> Self {
        let win = gtk::ApplicationWindowBuilder::new()
            .application(app)
            .build();
        set_visual(&win, None);

        let vte = vte::Terminal::new();

        let shell = if let Some(u) = pwd::Passwd::current_user() {
            u.shell
        } else {
            "/bin/sh".into()
        };

        {
            const PCRE2_MULTILINE: u32 = 1024;
            // const PCRE2_UTF: u32 = 524288;
            // const PCRE2_NO_UTF_CHECK: u32 = 1073741824;

            let regex = vte::Regex::new_for_match(LINK_EXPR, PCRE2_MULTILINE).unwrap();
            let tag = vte.match_add_regex(&regex, 0);

            vte.match_set_cursor_name(tag, "pointer");
        }

        // let fw = gdk::RGBA::white();
        let bg = gdk::RGBA {
            red: 0.0,
            green: 0.0,
            blue: 0.0,
            alpha: 0.7,
        };

        vte.set_color_background(&bg);

        // vte.set_colors(
        //     // Some(&fw),
        //     None,
        //     Some(&bg),
        //     &[
        //         &gdk::RGBA::red(),
        //         &gdk::RGBA::green(),
        //         &gdk::RGBA::blue(),
        //         &gdk::RGBA::red(),
        //         &gdk::RGBA::green(),
        //         &gdk::RGBA::blue(),
        //         &gdk::RGBA::red(),
        //         &gdk::RGBA::green(),
        //     ],
        // );

        let fd = pango::FontDescription::from_string("Monospace 9");
        vte.set_font(Some(&fd));

        vte.spawn_async::<gio::Cancellable>(
            vte::PtyFlags::DEFAULT,
            None,
            &[std::path::Path::new(&shell)],
            &[],
            glib::SpawnFlags::SEARCH_PATH,
            Some(Box::new(|| {})),
            -1,
            None,
            None,
        );

        win.add(&vte);

        win.show_all();

        Self {
            vte,
            win,
            state: Rc::new(RefCell::new(State::new())),
        }
    }

    pub fn connect(&mut self) {
        let v = self.vte.clone();
        self.vte.connect_button_press_event(move |_, e| {
            let mut e = e.clone();
            let (res, _) = v.match_check_event(&mut e);

            if let Some(res) = res {
                println!("{}", res);
                gtk::show_uri(None, &res.to_string(), 0).ok();
            }

            Inhibit(false)
        });

        let win = self.win.clone();
        self.vte.connect_child_exited(move |_, _| {
            win.close();
        });
    }
}

fn set_visual(window: &gtk::ApplicationWindow, _screen: Option<&gdk::Screen>) {
    if let Some(screen) = window.get_screen() {
        if let Some(ref visual) = screen.get_rgba_visual() {
            window.set_visual(Some(visual));
        }
    }
}
