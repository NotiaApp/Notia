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
        // Template widgets - updated to match the UI template
        #[template_child]
        pub gallery_grid: TemplateChild<gtk::GridView>,
        #[template_child]
        pub note_text_view: TemplateChild<gtk::TextView>,
        #[template_child]
        pub save_note_button: TemplateChild<gtk::Button>,
        #[template_child]
        pub photo_label: TemplateChild<gtk::Label>,
        #[template_child]
        pub note_label: TemplateChild<gtk::Label>,
        #[template_child]
        pub selected_photo_preview: TemplateChild<gtk::Picture>,
        #[template_child]
        pub photo_counter: TemplateChild<gtk::Label>,
        #[template_child]
        pub prev_button: TemplateChild<gtk::Button>,
        #[template_child]
        pub next_button: TemplateChild<gtk::Button>,
        #[template_child]
        pub clear_note_button: TemplateChild<gtk::Button>,
        #[template_child]
        pub toast_overlay: TemplateChild<adw::ToastOverlay>,
        
        pub photo_manager: Rc<RefCell<PhotoManager>>,
        pub current_photo_index: RefCell<usize>,
        pub gallery_model: RefCell<gio::ListStore>,
    }

    impl Default for NotiaWindow {
        fn default() -> Self {
            Self {
                gallery_grid: TemplateChild::default(),
                note_text_view: TemplateChild::default(),
                save_note_button: TemplateChild::default(),
                photo_label: TemplateChild::default(),
                note_label: TemplateChild::default(),
                selected_photo_preview: TemplateChild::default(),
                photo_counter: TemplateChild::default(),
                prev_button: TemplateChild::default(),
                next_button: TemplateChild::default(),
                clear_note_button: TemplateChild::default(),
                toast_overlay: TemplateChild::default(),
                photo_manager: Rc::new(RefCell::new(PhotoManager::new())),
                current_photo_index: RefCell::new(0),
                gallery_model: RefCell::new(gio::ListStore::new::<gio::File>()),
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
        
        // Gallery grid selection
        imp.gallery_grid.connect_activate(glib::clone!(
            #[weak(rename_to = window)]
            self,
            move |_, position| {
                window.on_photo_selected(position as usize);
            }
        ));
        
        // Navigation buttons
        imp.prev_button.connect_clicked(glib::clone!(
            #[weak(rename_to = window)]
            self,
            move |_| {
                window.navigate_previous();
            }
        ));
        
        imp.next_button.connect_clicked(glib::clone!(
            #[weak(rename_to = window)]
            self,
            move |_| {
                window.navigate_next();
            }
        ));
        
        // Note buttons
        imp.save_note_button.connect_clicked(glib::clone!(
            #[weak(rename_to = window)]
            self,
            move |_| {
                window.save_current_note();
            }
        ));
        
        imp.clear_note_button.connect_clicked(glib::clone!(
            #[weak(rename_to = window)]
            self,
            move |_| {
                window.clear_current_note();
            }
        ));
    }

    pub fn load_photos(&self) {
        let imp = self.imp();
        
        // Clear the gallery model
        imp.gallery_model.borrow_mut().remove_all();
        
        // Load photos
        {
            let mut manager = imp.photo_manager.borrow_mut();
            manager.scan_photos();
            
            for photo_path in &manager.photos {
                let file = gio::File::for_path(photo_path);
                imp.gallery_model.borrow().append(&file);
            }
        }
        
        // Setup the grid view
        let selection_model = gtk::SingleSelection::new(Some(imp.gallery_model.borrow().clone()));
        let factory = gtk::SignalListItemFactory::new();
        
        factory.connect_setup(|_, item| {
            let item = item.downcast_ref::<gtk::ListItem>().unwrap();
            let picture = gtk::Picture::new();
            picture.set_content_fit(gtk::ContentFit::Cover);
            picture.set_size_request(200, 150);
            item.set_child(Some(&picture));
        });
        
        factory.connect_bind(|_, item| {
            let item = item.downcast_ref::<gtk::ListItem>().unwrap();
            let picture = item.child().and_downcast::<gtk::Picture>().unwrap();
            let file = item.item().and_downcast::<gio::File>().unwrap();
            
            picture.set_file(Some(&file));
        });
        
        imp.gallery_grid.set_factory(Some(&factory));
        imp.gallery_grid.set_model(Some(&selection_model));
        
        // Update UI
        if imp.gallery_model.borrow().n_items() > 0 {
            *imp.current_photo_index.borrow_mut() = 0;
            self.update_current_photo();
        }
    }
    
    fn on_photo_selected(&self, index: usize) {
        let imp = self.imp();
        *imp.current_photo_index.borrow_mut() = index;
        self.update_current_photo();
    }
    
    fn navigate_previous(&self) {
        let imp = self.imp();
        let count = imp.gallery_model.borrow().n_items();
        if count == 0 {
            return;
        }
        
        let mut current = imp.current_photo_index.borrow_mut();
        if *current > 0 {
            *current -= 1;
            self.update_current_photo();
        }
    }
    
    fn navigate_next(&self) {
        let imp = self.imp();
        let count = imp.gallery_model.borrow().n_items();
        if count == 0 {
            return;
        }
        
        let mut current = imp.current_photo_index.borrow_mut();
        if *current < count as usize - 1 {
            *current += 1;
            self.update_current_photo();
        }
    }

    pub fn update_current_photo(&self) {
        let imp = self.imp();
        let current_index = *imp.current_photo_index.borrow();
        let count = imp.gallery_model.borrow().n_items();
        
        if count == 0 {
            imp.photo_label.set_text("No photos available");
            imp.note_label.set_text("No photos to add notes to");
            imp.photo_counter.set_text("0 / 0");
            imp.selected_photo_preview.set_paintable(None::<&gtk::gdk::Paintable>);
            return;
        }
        
        // Get the current photo file
        let file = imp.gallery_model.borrow().item(current_index as u32)
            .and_downcast::<gio::File>().unwrap();
        let photo_path = file.path().unwrap_or_default().to_string_lossy().to_string();
        
        // Update photo counter
        imp.photo_counter.set_text(&format!("{} / {}", current_index + 1, count));
        
        // Update photo preview
        imp.selected_photo_preview.set_file(Some(&file));
        
        // Get photo name
        let photo_name = std::path::Path::new(&photo_path)
            .file_name()
            .and_then(|name| name.to_str())
            .unwrap_or("Unknown File")
            .to_string();
        
        // Get note info
        let (note_text, note_status) = {
            let manager = imp.photo_manager.borrow();
            if let Some(note) = manager.get_note(&photo_path) {
                (note.note.clone(), format!("Note: {}", note.timestamp))
            } else {
                (String::new(), "No note added yet".to_string())
            }
        };
        
        // Update UI
        imp.photo_label.set_text(&photo_name);
        imp.note_label.set_text(&note_status);
        
        let buffer = imp.note_text_view.buffer();
        buffer.set_text(&note_text);
        
        // Update navigation button states
        imp.prev_button.set_sensitive(current_index > 0);
        imp.next_button.set_sensitive(current_index < count as usize - 1);
    }

    fn save_current_note(&self) {
        let imp = self.imp();
        let current_index = *imp.current_photo_index.borrow();
        let count = imp.gallery_model.borrow().n_items();
        
        if count == 0 {
            return;
        }
        
        // Get the current photo file
        let file = imp.gallery_model.borrow().item(current_index as u32)
            .and_downcast::<gio::File>().unwrap();
        let photo_path = file.path().unwrap_or_default().to_string_lossy().to_string();
        
        // Get note text
        let buffer = imp.note_text_view.buffer();
        let (start, end) = buffer.bounds();
        let note_text = buffer.text(&start, &end, false);
        
        // Save note
        if !note_text.trim().is_empty() {
            let mut manager = imp.photo_manager.borrow_mut();
            manager.add_note(&photo_path, note_text.to_string());
        }
        
        // Update UI
        self.update_current_photo();
        
        // Show toast
        let toast = adw::Toast::new("Note saved successfully");
        imp.toast_overlay.add_toast(toast);
    }
    
    fn clear_current_note(&self) {
        let imp = self.imp();
        let current_index = *imp.current_photo_index.borrow();
        let count = imp.gallery_model.borrow().n_items();
        
        if count == 0 {
            return;
        }
        
        // Get the current photo file
        let file = imp.gallery_model.borrow().item(current_index as u32)
            .and_downcast::<gio::File>().unwrap();
        let photo_path = file.path().unwrap_or_default().to_string_lossy().to_string();
        
        // Clear note
        let mut manager = imp.photo_manager.borrow_mut();
        manager.remove_note(&photo_path);
        
        // Clear text view
        let buffer = imp.note_text_view.buffer();
        buffer.set_text("");
        
        // Update UI
        self.update_current_photo();
        
        // Show toast
        let toast = adw::Toast::new("Note cleared");
        imp.toast_overlay.add_toast(toast);
    }

    pub fn clear_all_notes(&self) {
        let imp = self.imp();
        let mut manager = imp.photo_manager.borrow_mut();
        manager.clear_notes();
        
        // Clear text view if it contains a note for the current photo
        let buffer = imp.note_text_view.buffer();
        let (start, end) = buffer.bounds();
        if !buffer.text(&start, &end, false).is_empty() {
            buffer.set_text("");
        }
        
        self.update_current_photo();
        
        // Show toast
        let toast = adw::Toast::new("All notes cleared");
        imp.toast_overlay.add_toast(toast);
    }
}
