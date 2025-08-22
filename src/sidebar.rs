use gtk::prelude::*;
use gtk::subclass::prelude::*;
use gtk::{glib, CompositeTemplate};
use std::cell::RefCell;
use std::rc::Rc;
use crate::photo_manager::PhotoManager;

pub struct SidebarData {
    pub photo_path: Option<String>,
    pub photo_name: Option<String>,
    pub note_text: Option<String>,
    pub note_status: Option<String>,
    pub tags: Option<Vec<String>>,
}

mod imp {
    use super::*;

    #[derive(Debug, Default, CompositeTemplate)]
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
        
        pub photo_manager: RefCell<Option<Rc<RefCell<PhotoManager>>>>,
        pub current_photo_path: RefCell<Option<String>>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for Sidebar {
        const NAME: &'static str = "Sidebar";
        type Type = super::Sidebar;
        type ParentType = gtk::Box;

        fn class_init(klass: &mut Self::Class) {
            klass.bind_template();
        }

        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
            obj.init_template();
        }
    }

    impl ObjectImpl for Sidebar {}
    impl WidgetImpl for Sidebar {}
    impl BoxImpl for Sidebar {}
}

glib::wrapper! {
    pub struct Sidebar(ObjectSubclass<imp::Sidebar>) @extends gtk::Widget, gtk::Box;
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
        
        // Etiketleri güncelle
        if let Some(tags) = data.tags {
            // Mevcut etiketleri temizle
            while let Some(child) = imp.tag_chip_box.first_child() {
                imp.tag_chip_box.remove(&child);
            }
            
            // Yeni etiketleri ekle
            for tag in tags {
                self.add_tag_chip(&tag);
            }
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
        let entry = imp.tag_entry.clone();
        let add_btn = imp.add_tag_button.clone();
        let sidebar = self.clone();
        add_btn.connect_clicked(move |_| {
            let text = entry.text().to_string();
            if !text.trim().is_empty() {
                sidebar.add_tag_chip(text.trim());
                entry.set_text("");
            }
        });
    }

    pub fn set_photo_manager(&self, manager: Rc<RefCell<PhotoManager>>) {
        let imp = self.imp();
        *imp.photo_manager.borrow_mut() = Some(manager);
    }

    pub fn set_current_photo_path(&self, photo_path: String) {
        let imp = self.imp();
        *imp.current_photo_path.borrow_mut() = Some(photo_path);
    }

    pub fn get_current_tags(&self) -> Vec<String> {
        let imp = self.imp();
        let mut tags = Vec::new();
        
        // Get tags from UI chips
        let mut child = imp.tag_chip_box.first_child();
        while let Some(widget) = child {
            if let Some(chip) = widget.downcast_ref::<gtk::Box>() {
                if let Some(label) = chip.first_child().and_downcast::<gtk::Label>() {
                    let tag_text = label.text();
                    tags.push(tag_text.to_string());
                }
            }
            child = widget.next_sibling();
        }
        
        tags
    }

    pub fn add_tag_chip(&self, tag: &str) {
        let imp = self.imp();
        let chip = gtk::Box::new(gtk::Orientation::Horizontal, 2);
        chip.add_css_class("tag-chip");
        
        let label = gtk::Label::new(Some(tag));
        label.add_css_class("tag-label");
        
        // Basit renk seçimi
        let _color = Self::get_tag_color(tag);
        label.set_margin_top(2);
        label.set_margin_bottom(2);
        label.set_margin_start(4);
        label.set_margin_end(4);
        
        // Silme butonu
        let del_btn = gtk::Button::from_icon_name("window-close-symbolic");
        del_btn.add_css_class("tag-delete-btn");
        del_btn.set_focusable(false);
        
        let chip_clone = chip.clone();
        let sidebar = self.clone();
        let tag_str = tag.to_string();
        del_btn.connect_clicked(move |_| {
            chip_clone.unparent();
            // Etiketi PhotoManager'dan da sil
            sidebar.remove_tag_from_manager(&tag_str);
        });
        
        chip.append(&label);
        chip.append(&del_btn);
        imp.tag_chip_box.append(&chip);
        
        // Etiketi PhotoManager'a ekle
        self.add_tag_to_manager(tag);
    }

    fn add_tag_to_manager(&self, tag: &str) {
        let imp = self.imp();
        if let Some(manager) = imp.photo_manager.borrow().as_ref() {
            if let Some(photo_path) = imp.current_photo_path.borrow().as_ref() {
                let mut manager = manager.borrow_mut();
                manager.add_tag(photo_path, tag.to_string());
            }
        }
    }

    fn remove_tag_from_manager(&self, tag: &str) {
        let imp = self.imp();
        if let Some(manager) = imp.photo_manager.borrow().as_ref() {
            if let Some(photo_path) = imp.current_photo_path.borrow().as_ref() {
                let mut manager = manager.borrow_mut();
                manager.remove_tag(photo_path, tag);
            }
        }
    }

    fn get_tag_color(tag: &str) -> String {
        // Basit hash-based renk seçimi
        let colors = [
            "#FFB300", "#803E75", "#FF6800", "#A6BDD7", "#C10020", "#CEA262", "#817066",
            "#007D34", "#F6768E", "#00538A", "#FF7A5C", "#53377A", "#FF8E00", "#B32851",
            "#F4C800", "#7F180D", "#93AA00", "#593315", "#F13A13", "#232C16"
        ];
        
        let hash = tag.chars().map(|c| c as u32).sum::<u32>();
        let index = (hash % colors.len() as u32) as usize;
        colors[index].to_string()
    }
}
