use gtk4::{glib, prelude::*};

fn main() -> glib::ExitCode {
    let application = gtk4::Application::builder()
        .application_id("com.github.gtk-rs.examples.basic")
        .build();
    application.connect_activate(build_ui);
    application.run()
}

fn build_ui(application: &gtk4::Application) {
    let window = gtk4::ApplicationWindow::new(application);

    window.set_title(Some("First GTK Program"));
    window.set_default_size(350, 70);

    let button = gtk4::Button::with_label("Click me!");

    window.set_child(Some(&button));

    window.present();
}
