use gtk::prelude::*;
use gtk::subclass::prelude::*;
use gtk::{glib, CompositeTemplate};

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

// Public wrapper
glib::wrapper! {
    pub struct Sidebar(ObjectSubclass<Sidebar>) @extends gtk::Widget, gtk::Revealer;
}

impl Sidebar {
    pub fn new() -> Self {
        glib::Object::builder().build()
    }
}
