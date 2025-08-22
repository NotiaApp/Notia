// window.rs
use gtk::prelude::*;
use adw::subclass::prelude::*;
use gtk::{gio, glib};
use gettextrs::gettext;
use crate::photo_manager::PhotoManager;
use crate::sidebar::Sidebar;

mod imp {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    #[derive(Debug, gtk::CompositeTemplate)]
    #[template(resource = "/com/vastsea/notia/window.ui")]
    pub struct NotiaWindow {
        // Template widgets - reordered to match the UI template
        #[template_child]
        pub toast_overlay: TemplateChild<adw::ToastOverlay>,
        #[template_child]
        pub sidebar_toggle: TemplateChild<gtk::ToggleButton>,
        #[template_child]
        pub gallery_grid: TemplateChild<gtk::GridView>,
        #[template_child]
        pub prev_button: TemplateChild<gtk::Button>,
        #[template_child]
        pub photo_counter: TemplateChild<gtk::Label>,
        #[template_child]
        pub next_button: TemplateChild<gtk::Button>,
        #[template_child]
        pub sidebar_revealer: TemplateChild<gtk::Revealer>,
        
        pub photo_manager: Rc<RefCell<PhotoManager>>,
        pub current_photo_index: RefCell<usize>,
        pub gallery_model: RefCell<gio::ListStore>,
        pub sidebar: RefCell<Option<Sidebar>>,
    }

     impl Default for NotiaWindow {
        fn default() -> Self {
            Self {
                toast_overlay: TemplateChild::default(),
                sidebar_toggle: TemplateChild::default(),
                gallery_grid: TemplateChild::default(),
                prev_button: TemplateChild::default(),
                photo_counter: TemplateChild::default(),
                next_button: TemplateChild::default(),
                sidebar_revealer: TemplateChild::default(),
                photo_manager: Rc::new(RefCell::new(PhotoManager::new())),
                current_photo_index: RefCell::new(0),
                gallery_model: RefCell::new(gio::ListStore::new::<gio::File>()),
                sidebar: RefCell::new(None),
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
        
        // Initialize sidebar
        let sidebar = Sidebar::new();
        sidebar.setup_tag_feature();
        
        // Set photo manager and current photo path
        sidebar.set_photo_manager(imp.photo_manager.clone());
        
        // Connect sidebar callbacks
        let window = self.clone();
        sidebar.connect_save_note(move || {
            window.save_current_note();
        });
        
        let window = self.clone();
        sidebar.connect_clear_note(move || {
            window.clear_current_note();
        });
        
        imp.sidebar_revealer.set_child(Some(&sidebar));
        *imp.sidebar.borrow_mut() = Some(sidebar.clone());
        
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
        
        // Sidebar toggle
        imp.sidebar_toggle.connect_active_notify(glib::clone!(
            #[weak(rename_to = window)]
            self,
            move |toggle| {
                let imp = window.imp();
                let active = toggle.is_active();
                imp.sidebar_revealer.set_reveal_child(active);
                imp.sidebar_revealer.set_hexpand(active);
                imp.sidebar_revealer.set_vexpand(active);
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
        // imp.save_note_button.connect_clicked(glib::clone!(
        //     #[weak(rename_to = window)]
        //     self,
        //     move |_| {
        //         window.save_current_note();
        //     }
        // ));
        
        // imp.clear_note_button.connect_clicked(glib::clone!(
        //     #[weak(rename_to = window)]
        //     self,
        //     move |_| {
        //         window.clear_current_note();
        //     }
        // ));
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
            imp.photo_counter.set_text("0 / 0");
            // Update sidebar with empty data
            if let Some(sidebar) = imp.sidebar.borrow().as_ref() {
                sidebar.update_sidebar(crate::sidebar::SidebarData {
                    photo_path: None,
                    photo_name: Some(gettext("No photos available")),
                    note_text: Some("".to_string()),
                    note_status: Some(gettext("No photos to add notes to")),
                    tags: None,
                });
            }
            return;
        }
        
        // Get the current photo file
        let file = imp.gallery_model.borrow().item(current_index as u32)
            .and_downcast::<gio::File>().unwrap();
        let photo_path = file.path().unwrap_or_default().to_string_lossy().to_string();
        
        // Update photo counter
        imp.photo_counter.set_text(&format!("{} / {}", current_index + 1, count));
        
        // Get photo name
        let photo_name = std::path::Path::new(&photo_path)
            .file_name()
            .and_then(|name| name.to_str())
            .unwrap_or(&gettext("Unknown File"))
            .to_string();
        
        // Get note info
        let (note_text, note_status) = {
            let manager = imp.photo_manager.borrow();
            if let Some(note) = manager.get_note(&photo_path) {
                (note.note.clone(), format!("{}: {}", gettext("Note"), note.timestamp))
            } else {
                (String::new(), gettext("No note added yet"))
            }
        };
        
        // Update sidebar with photo data
        if let Some(sidebar) = imp.sidebar.borrow().as_ref() {
            let tags = {
                let manager = imp.photo_manager.borrow();
                manager.get_tags(&photo_path)
            };
            
            // Set current photo path for tag operations
            sidebar.set_current_photo_path(photo_path.clone());
            
            sidebar.update_sidebar(crate::sidebar::SidebarData {
                photo_path: Some(photo_path.clone()),
                photo_name: Some(photo_name.clone()),
                note_text: Some(note_text.clone()),
                note_status: Some(note_status.clone()),
                tags: Some(tags),
            });
        }
        
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
        
        // Get note text from sidebar
        if let Some(sidebar) = imp.sidebar.borrow().as_ref() {
            let note_text = sidebar.get_note_text();
            
            // Save note while preserving existing tags
            if !note_text.trim().is_empty() {
                let mut manager = imp.photo_manager.borrow_mut();
                
                // Get current tags from sidebar (including newly added ones)
                let current_tags = sidebar.get_current_tags();
                
                // Create new note with current tags
                let photo_note = crate::photo_manager::PhotoNote {
                    path: photo_path.clone(),
                    note: note_text.to_string(),
                    timestamp: chrono::Utc::now().to_rfc3339(),
                    tags: current_tags,
                };
                manager.notes.insert(photo_path, photo_note);
                manager.save_notes();
            }
        }
        
        // Show toast
        let toast = adw::Toast::new(&gettext("Note saved successfully"));
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
        {
            let mut manager = imp.photo_manager.borrow_mut();
            manager.remove_note(&photo_path);
        } // borrow burada biter
        
        // Clear text view in sidebar
        if let Some(sidebar) = imp.sidebar.borrow().as_ref() {
            sidebar.clear_note_text();
        }
        
        // Update UI
        self.update_current_photo();
        
        // Show toast
        let toast = adw::Toast::new(&gettext("Note cleared"));
        imp.toast_overlay.add_toast(toast);
    }

    pub fn clear_all_notes(&self) {
        let imp = self.imp();
        let mut manager = imp.photo_manager.borrow_mut();
        manager.clear_notes();
        
        // Clear text view if it contains a note for the current photo
        // let buffer = imp.note_text_view.buffer();
        // let (start, end) = buffer.bounds();
        // if !buffer.text(&start, &end, false).is_empty() {
        //     buffer.set_text("");
        // }
        
        self.update_current_photo();
        
        // Show toast
        let toast = adw::Toast::new(&gettext("All notes cleared"));
        imp.toast_overlay.add_toast(toast);
    }
}
