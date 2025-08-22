use gtk::prelude::*;
use gtk::subclass::prelude::*;
use gtk::{glib, CompositeTemplate};

pub struct SidebarData {
    pub photo_path: Option<String>,
    pub photo_name: Option<String>,
    pub note_text: Option<String>,
    pub note_status: Option<String>,
}

#[derive(Debug, CompositeTemplate)]
#[template(resource = "/com/vastsea/notia/sidebar.ui")]
pub struct Sidebar {
    #[template_child]
    pub selected_photo_preview: TemplateChild<gtk::Picture>,
    #[template_child]
    pub photo_label: TemplateChild<gtk::Label>,
    #[template_child]
    pub note_label: TemplateChild<gtk::Label>,
    #[template_child]
    pub note_text_view: TemplateChild<gtk::TextView>,
    #[template_child]
    pub clear_note_button: TemplateChild<gtk::Button>,
    #[template_child]
    pub save_note_button: TemplateChild<gtk::Button>,
}

#[glib::object_subclass]
impl ObjectSubclass for Sidebar {
    const NAME: &'static str = "Sidebar";
    type Type = super::Sidebar;
    type ParentType = gtk::Revealer;

    fn class_init(klass: &mut Self::Class) {
        klass.bind_template();
    }

    fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
        obj.init_template();
    }
}

impl ObjectImpl for Sidebar {}
impl WidgetImpl for Sidebar {}
impl RevealerImpl for Sidebar {}

glib::wrapper! {
    pub struct Sidebar(ObjectSubclass<Sidebar>) @extends gtk::Widget, gtk::Revealer;
}

impl Sidebar {
    pub fn new() -> Self {
        glib::Object::builder().build()
    }

    pub fn update_sidebar(&self, data: SidebarData) {
        let imp = self.imp();
        if let Some(photo_name) = data.photo_name {
            imp.photo_label.set_text(&photo_name);
        }
        if let Some(note_status) = data.note_status {
            imp.note_label.set_text(&note_status);
        }
        if let Some(note_text) = data.note_text {
            let buffer = imp.note_text_view.buffer();
            buffer.set_text(&note_text);
        }
        // Fotoğraf önizlemesi için photo_path kullanılabilir
        if let Some(photo_path) = data.photo_path {
            let file = gio::File::for_path(photo_path);
            imp.selected_photo_preview.set_file(Some(&file));
        }
    }

    pub fn get_note_text(&self) -> String {
        let imp = self.imp();
        let buffer = imp.note_text_view.buffer();
        let (start, end) = buffer.bounds();
        buffer.text(&start, &end, false).to_string()
    }

    pub fn clear_note_text(&self) {
        let imp = self.imp();
        let buffer = imp.note_text_view.buffer();
        buffer.set_text("");
    }

    pub fn connect_save_note<F: Fn() + 'static>(&self, callback: F) {
        let imp = self.imp();
        imp.save_note_button.connect_clicked(move |_| {
            callback();
        });
    }

    pub fn connect_clear_note<F: Fn() + 'static>(&self, callback: F) {
        let imp = self.imp();
        imp.clear_note_button.connect_clicked(move |_| {
            callback();
        });
    }
}
