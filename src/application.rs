// application.rs
use gettextrs::gettext;
use adw::prelude::*;
use adw::subclass::prelude::*;
use gtk::{gio, glib};
use crate::config::VERSION;
use crate::NotiaWindow;

mod imp {
    use super::*;
    #[derive(Debug, Default)]
    pub struct NotiaApplication {}

    #[glib::object_subclass]
    impl ObjectSubclass for NotiaApplication {
        const NAME: &'static str = "NotiaApplication";
        type Type = super::NotiaApplication;
        type ParentType = adw::Application;
    }

    impl ObjectImpl for NotiaApplication {
        fn constructed(&self) {
            self.parent_constructed();
            let obj = self.obj();
            obj.setup_gactions();
            obj.set_accels_for_action("app.quit", &["<primary>q"]);
            obj.set_accels_for_action("app.refresh", &["<primary>r"]);
        }
    }

    impl ApplicationImpl for NotiaApplication {
        fn activate(&self) {
            let application = self.obj();
            let window = application.active_window().unwrap_or_else(|| {
                let window = NotiaWindow::new(&*application);
                window.upcast()
            });
            window.present();
        }
    }

    impl GtkApplicationImpl for NotiaApplication {}
    impl AdwApplicationImpl for NotiaApplication {}
}

glib::wrapper! {
    pub struct NotiaApplication(ObjectSubclass<imp::NotiaApplication>)
        @extends gio::Application, gtk::Application, adw::Application,
        @implements gio::ActionGroup, gio::ActionMap;
}

impl NotiaApplication {
    pub fn new(application_id: &str, flags: &gio::ApplicationFlags) -> Self {
        glib::Object::builder()
            .property("application-id", application_id)
            .property("flags", flags)
            .property("resource-base-path", "/com/vastsea/notia")
            .build()
    }

    fn setup_gactions(&self) {
        let quit_action = gio::ActionEntry::builder("quit")
            .activate(move |app: &Self, _, _| app.quit())
            .build();

        let about_action = gio::ActionEntry::builder("about")
            .activate(move |app: &Self, _, _| app.show_about())
            .build();

        let refresh_action = gio::ActionEntry::builder("refresh")
            .activate(move |app: &Self, _, _| {
                if let Some(window) = app.active_window() {
                    if let Some(notia_window) = window.downcast_ref::<NotiaWindow>() {
                        notia_window.load_photos();
                    }
                }
            })
            .build();

        let clear_notes_action = gio::ActionEntry::builder("clear_notes")
            .activate(move |app: &Self, _, _| {
                if let Some(window) = app.active_window() {
                    if let Some(notia_window) = window.downcast_ref::<NotiaWindow>() {
                        notia_window.clear_all_notes();
                    }
                }
            })
            .build();

        self.add_action_entries([quit_action, about_action, refresh_action, clear_notes_action]);
    }

    fn show_about(&self) {
        let window = self.active_window().unwrap();
        let about = adw::AboutDialog::builder()
            .application_name("Notia")
            .application_icon("com.vastsea.notia")
            .developer_name("Notia Team")
            .version(VERSION)
            .developers(vec!["Notia Team"])
            .translator_credits(&gettext("translator-credits"))
            .copyright("© 2025 Notia Team")
            .website("https://github.com/yourusername/notia")
            .build();
        about.present(Some(&window));
    }
}
