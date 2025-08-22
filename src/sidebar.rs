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
    #[template_child]
    pub tag_chip_box: TemplateChild<gtk::Box>,
    #[template_child]
    pub tag_entry: TemplateChild<gtk::Entry>,
    #[template_child]
    pub add_tag_button: TemplateChild<gtk::Button>,
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

    pub fn setup_tag_feature(&self) {
        let imp = self.imp();
        let tag_box = imp.tag_chip_box.clone();
        let entry = imp.tag_entry.clone();
        let add_btn = imp.add_tag_button.clone();
        let sidebar = self.clone();
        add_btn.connect_clicked(move |_| {
            if let Some(text) = entry.text().as_str().strip_prefix(' ') {
                if !text.trim().is_empty() {
                    sidebar.add_tag_chip(text.trim());
                    entry.set_text("");
                }
            } else {
                let text = entry.text().to_string();
                if !text.trim().is_empty() {
                    sidebar.add_tag_chip(text.trim());
                    entry.set_text("");
                }
            }
        });
    }

    pub fn add_tag_chip(&self, tag: &str) {
        let imp = self.imp();
        let chip = gtk::Box::new(gtk::Orientation::Horizontal, 2);
        chip.set_widget_name("tag_chip");
        let label = gtk::Label::new(Some(tag));
        label.add_css_class("tag-label");
        // Renkli chip için rastgele bir renk seç
        use gtk::gdk::RGBA;
        let color = RGBA::parse(&Self::random_color()).unwrap_or(RGBA::RED);
        let style = format!("background-color: {}; color: white; border-radius: 8px; padding: 2px 8px; margin: 2px;", color.to_string());
        label.set_css_classes(&["tag-label"]);
        label.set_style_context(&gtk::StyleContext::new());
        label.set_widget_name("tag_label");
        label.set_margin_top(2);
        label.set_margin_bottom(2);
        label.set_margin_start(4);
        label.set_margin_end(4);
        // Silme butonu
        let del_btn = gtk::Button::from_icon_name("window-close-symbolic");
        del_btn.set_widget_name("tag_delete_btn");
        del_btn.set_relief(gtk::ReliefStyle::None);
        del_btn.set_focusable(false);
        let chip_clone = chip.clone();
        del_btn.connect_clicked(move |_| {
            chip_clone.set_parent(None::<&gtk::Widget>);
        });
        chip.append(&label);
        chip.append(&del_btn);
        imp.tag_chip_box.append(&chip);
    }

    fn random_color() -> String {
        // Basit pastel renkler
        let colors = [
            "#FFB300", "#803E75", "#FF6800", "#A6BDD7", "#C10020", "#CEA262", "#817066",
            "#007D34", "#F6768E", "#00538A", "#FF7A5C", "#53377A", "#FF8E00", "#B32851",
            "#F4C800", "#7F180D", "#93AA00", "#593315", "#F13A13", "#232C16"
        ];
        use rand::seq::SliceRandom;
        let mut rng = rand::thread_rng();
        colors.choose(&mut rng).unwrap_or(&"#FFB300").to_string()
    }
}
