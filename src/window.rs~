// window.rs
use gtk::prelude::*;
use adw::subclass::prelude::*;
use gtk::{gio, glib};
use crate::photo_manager::PhotoManager;

mod imp {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    #[derive(Debug, gtk::CompositeTemplate)]
    #[template(resource = "/com/vastsea/notia/window.ui")]
    pub struct NotiaWindow {
        // Template widgets
        #[template_child]
        pub carousel: TemplateChild<adw::Carousel>,
        #[template_child]
        pub note_text_view: TemplateChild<gtk::TextView>,
        #[template_child]
        pub save_note_button: TemplateChild<gtk::Button>,
        #[template_child]
        pub photo_label: TemplateChild<gtk::Label>,
        #[template_child]
        pub note_label: TemplateChild<gtk::Label>,

        pub photo_manager: Rc<RefCell<PhotoManager>>,
        pub current_photo_index: RefCell<usize>,
    }

    impl Default for NotiaWindow {
        fn default() -> Self {
            Self {
                carousel: TemplateChild::default(),
                note_text_view: TemplateChild::default(),
                save_note_button: TemplateChild::default(),
                photo_label: TemplateChild::default(),
                note_label: TemplateChild::default(),
                photo_manager: Rc::new(RefCell::new(PhotoManager::new())),
                current_photo_index: RefCell::new(0),
            }
        }
    }

    #[glib::object_subclass]
    impl ObjectSubclass for NotiaWindow {
        const NAME: &'static str = "NotiaWindow";
        type Type = super::NotiaWindow;
        type ParentType = adw::ApplicationWindow;
        fn class_init(klass: &mut Self::Class) {
            klass.bind_template();
        }
        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
            obj.init_template();
        }
    }

    impl ObjectImpl for NotiaWindow {
        fn constructed(&self) {
            self.parent_constructed();
            let obj = self.obj();
            obj.setup_callbacks();
            obj.load_photos();
        }
    }

    impl WidgetImpl for NotiaWindow {}
    impl WindowImpl for NotiaWindow {}
    impl ApplicationWindowImpl for NotiaWindow {}
    impl AdwApplicationWindowImpl for NotiaWindow {}
}

glib::wrapper! {
    pub struct NotiaWindow(ObjectSubclass<imp::NotiaWindow>)
        @extends gtk::Widget, gtk::Window, gtk::ApplicationWindow, adw::ApplicationWindow,
        @implements gio::ActionGroup, gio::ActionMap;
}

impl NotiaWindow {
    pub fn new<P: IsA<gtk::Application>>(application: &P) -> Self {
        glib::Object::builder()
            .property("application", application)
            .build()
    }

    fn setup_callbacks(&self) {
        let imp = self.imp();

        // Carousel değiştiğinde notları güncelle
        imp.carousel.connect_position_notify(glib::clone!(
            #[weak(rename_to = window)]
            self,
            move |_| {
                window.update_current_photo();
            }
        ));

        // Not kaydet butonu
        imp.save_note_button.connect_clicked(glib::clone!(
            #[weak(rename_to = window)]
            self,
            move |_| {
                window.save_current_note();
            }
        ));
    }

    pub fn load_photos(&self) {
        let imp = self.imp();

        // Carousel'ı temizle
        while let Some(child) = imp.carousel.first_child() {
            imp.carousel.remove(&child);
        }

        // Fotoğrafları yükle
        {
            let mut manager = imp.photo_manager.borrow_mut();
            manager.scan_photos();

            for photo_path in &manager.photos {
                let picture = gtk::Picture::for_filename(photo_path);
                picture.set_halign(gtk::Align::Center);
                picture.set_valign(gtk::Align::Center);
                picture.set_size_request(400, 300);

                let scrolled = gtk::ScrolledWindow::new();
                scrolled.set_child(Some(&picture));
                scrolled.set_policy(gtk::PolicyType::Automatic, gtk::PolicyType::Automatic);

                imp.carousel.append(&scrolled);
            }
        }

        *imp.current_photo_index.borrow_mut() = 0;
        self.update_current_photo();
    }

    pub fn update_current_photo(&self) {
        let imp = self.imp();
        let current_index = imp.carousel.position() as usize;

        // Önce mevcut indeksi güncelle
        *imp.current_photo_index.borrow_mut() = current_index;

        // Tüm bilgileri tek bir borrow işlemiyle al
        let (photo_path, photo_name, note_info) = {
            let manager = imp.photo_manager.borrow();

            if current_index >= manager.photos.len() {
                return;
            }

            let photo_path = manager.photos[current_index].clone();

            // Fotoğraf adını al
            let photo_name = std::path::Path::new(&photo_path)
                .file_name()
                .and_then(|name| name.to_str())
                .unwrap_or("Bilinmeyen Dosya")
                .to_string();

            // Not bilgisini al
            let note_info = if let Some(note) = manager.get_note(&photo_path) {
                (note.note.clone(), format!("Not: {}", note.timestamp))
            } else {
                (String::new(), "Henüz not eklenmemiş".to_string())
            };

            (photo_path, photo_name, note_info)
        };

        // UI'ı güncelle
        imp.photo_label.set_text(&photo_name);
        imp.note_label.set_text(&note_info.1);

        let buffer = imp.note_text_view.buffer();
        buffer.set_text(&note_info.0);
    }

    fn save_current_note(&self) {
        let imp = self.imp();
        let current_index = *imp.current_photo_index.borrow();

        // Mevcut fotoğraf yolunu ve not metnini al
        let (photo_path, note_text) = {
            let manager = imp.photo_manager.borrow();

            if current_index >= manager.photos.len() {
                return;
            }

            let photo_path = manager.photos[current_index].clone();

            let buffer = imp.note_text_view.buffer();
            let (start, end) = buffer.bounds();
            let note_text = buffer.text(&start, &end, false);

            (photo_path, note_text)
        };

        // Notu kaydet (ayrı bir borrow işlemi)
        if !note_text.trim().is_empty() {
            let mut manager = imp.photo_manager.borrow_mut();
            manager.add_note(&photo_path, note_text.to_string());
        }

        // UI'ı güncelle
        self.update_current_photo();
    }

    pub fn clear_all_notes(&self) {
        let imp = self.imp();
        let mut manager = imp.photo_manager.borrow_mut();
        manager.clear_notes();
        self.update_current_photo();
    }
}
