extern crate gdk;
extern crate gio;
extern crate gtk;

use gio::prelude::*;
use gtk::prelude::*;
use std::{env::args};

fn main() {
    let application = gtk::Application::new(
        Some("rust_gtk"),
        gio::ApplicationFlags::empty()
    ).expect("Errors in building application");

    application.connect_startup(
        |app| {
            let window = gtk::ApplicationWindow::new(app);

            window.set_title("Rust GTK");
            window.set_default_size(300, 200);
            window.connect_delete_event(move |win, _| {
                win.destroy();
                Inhibit(false)
            });

            let label = gtk::Label::new(Some("Hello"));
            window.add(&label);
            window.show_all();
        }
    );

    application.connect_activate(|_| {});
    application.run(&args().collect::<Vec<_>>());
}

// install GTK dependency: https://www.gtk.org/download/windows.php
// https://blogs.gnome.org/nacho/2014/08/01/how-to-build-your-gtk-application-on-windows/
