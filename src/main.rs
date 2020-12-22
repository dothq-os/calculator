extern crate gtk;
extern crate gio;

use gtk::prelude::*;
use gio::prelude::*;

use gtk::{Application, Entry};

static LAYOUT_GLADE: &str = include_str!("layout.glade");

fn build_ui(application: &gtk::Application) {
    let builder = gtk::Builder::from_string(LAYOUT_GLADE);

    let window: gtk::Window = builder.get_object("calculator").unwrap();
    // let button: gtk::Button = builder.get_object("button1").unwrap();
    // let dialog: gtk::MessageDialog = builder.get_object("messagedialog1").unwrap();

    // button.connect_clicked(move |_| {
    //     dialog.run();
    //     dialog.hide();
    // });

    window.show_all();
    gtk::main();
}

fn main() {
    let application = Application::new(
        Some("co.dothq.gtk-demo"),
        Default::default()
    ).expect("failed to initialize GTK application");

    application.connect_activate(move |app| {
        build_ui(app);
    });

    application.run(&[]);
}
