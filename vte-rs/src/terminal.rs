use crate::Terminal;
use glib::object::IsA;
use glib::translate::{IntoGlib, ToGlibPtr};

pub trait TerminalExtManual: 'static {
    fn watch_child(&self, child_pid: glib::Pid);
    fn set_colors(
        &self,
        foreground: Option<&gdk::RGBA>,
        background: Option<&gdk::RGBA>,
        palette: &[&gdk::RGBA],
    );
}

impl<O: IsA<Terminal>> TerminalExtManual for O {
    fn watch_child(&self, child_pid: glib::Pid) {
        unsafe {
            ffi::vte_terminal_watch_child(self.as_ref().to_glib_none().0, child_pid.into_glib());
        }
    }

    fn set_colors(
        &self,
        foreground: Option<&gdk::RGBA>,
        background: Option<&gdk::RGBA>,
        palette: &[&gdk::RGBA],
    ) {
        let palette_size = palette.len() as usize;

        let mut list = Vec::new();

        for p in palette {
            list.push(p.to_glib_none().0);
        }

        unsafe {
            ffi::vte_terminal_set_colors(
                self.as_ref().to_glib_none().0,
                foreground.to_glib_none().0,
                background.to_glib_none().0,
                // palette.to_glib_none().0,
                *list.as_ptr(),
                palette_size,
            );
        }
    }
}
