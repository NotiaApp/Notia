// main.rs
mod application;
mod config;
mod window;
mod sidebar;
mod photo_manager;
use self::application::NotiaApplication;
use self::window::NotiaWindow;
use self::sidebar::Sidebar;
use config::{GETTEXT_PACKAGE, LOCALEDIR, PKGDATADIR};
use gettextrs::{bind_textdomain_codeset, bindtextdomain, textdomain};
use gtk::{gio, glib};
use gtk::prelude::*;

fn main() -> glib::ExitCode {
    // Set English as default language
    std::env::set_var("LANG", "en_US.UTF-8");
    std::env::set_var("LC_ALL", "en_US.UTF-8");
    
    // Initialize libadwaita
    adw::init().expect("Failed to initialize libadwaita");
    
    // Set up gettext translations
    bindtextdomain(GETTEXT_PACKAGE, LOCALEDIR).expect("Unable to bind the text domain");
    bind_textdomain_codeset(GETTEXT_PACKAGE, "UTF-8")
        .expect("Unable to set the text domain encoding");
    textdomain(GETTEXT_PACKAGE).expect("Unable to switch to the text domain");
    
    // Load resources
    let resource_path = if std::path::Path::new("build/src/notia.gresource").exists() {
        "build/src/notia.gresource".to_string()
    } else {
        PKGDATADIR.to_owned() + "/notia.gresource"
    };
    let resources = gio::Resource::load(resource_path)
        .expect("Could not load resources");
    gio::resources_register(&resources);
    
    // Load CSS
    load_css();
    
    // Create a new GtkApplication with libadwaita
    let app = NotiaApplication::new("com.vastsea.notia", &gio::ApplicationFlags::empty());
    
    // Set up the style manager for dark/light theme support
    let style_manager = adw::StyleManager::default();
    style_manager.set_color_scheme(adw::ColorScheme::PreferLight);
    
    // Run the application
    app.run()
}

fn load_css() {
    // Create a CSS provider
    let css_provider = gtk::CssProvider::new();
    
    // Load CSS from resource
    css_provider.load_from_resource("/com/vastsea/notia/style.css");
    
    // Get the default display
    let display = &gtk::gdk::Display::default().expect("Could not get default display");
    
    // Add the CSS provider to the default display
    gtk::style_context_add_provider_for_display(
        display,
        &css_provider,
        gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
    );
}
