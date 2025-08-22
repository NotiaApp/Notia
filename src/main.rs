// main.rs
mod application;
mod config;
mod window;
mod photo_manager;

use self::application::NotiaApplication;
use self::window::NotiaWindow;
use config::{GETTEXT_PACKAGE, LOCALEDIR, PKGDATADIR};
use gettextrs::{bind_textdomain_codeset, bindtextdomain, textdomain};
use gtk::{gio, glib};
use gtk::prelude::*;

fn main() -> glib::ExitCode {
    // Initialize libadwaita
    adw::init().expect("Failed to initialize libadwaita");

    // Set up gettext translations
    bindtextdomain(GETTEXT_PACKAGE, LOCALEDIR).expect("Unable to bind the text domain");
    bind_textdomain_codeset(GETTEXT_PACKAGE, "UTF-8")
        .expect("Unable to set the text domain encoding");
    textdomain(GETTEXT_PACKAGE).expect("Unable to switch to the text domain");

    // Load resources
    let resources = gio::Resource::load(PKGDATADIR.to_owned() + "/notia.gresource")
        .expect("Could not load resources");
    gio::resources_register(&resources);

    // Create a new GtkApplication with libadwaita
    let app = NotiaApplication::new("com.vastsea.notia", &gio::ApplicationFlags::empty());

    // Set up the style manager for dark/light theme support
    let style_manager = adw::StyleManager::default();
    style_manager.set_color_scheme(adw::ColorScheme::PreferLight);

    // Run the application
    app.run()
}
