use gio;
use gio::prelude::*;
use gtk::prelude::*;
use gtk::EntryExt;
use gtk::LabelExt;

mod lib;

#[derive(Debug, Clone)]
struct AppSearchResults {
    text_boxes: [gtk::Label; 5],
}

impl AppSearchResults {
    pub fn new(search_query: &str) -> Self {
        let mut text_boxes: [gtk::Label; 5] = [
            gtk::Label::new(None),
            gtk::Label::new(None),
            gtk::Label::new(None),
            gtk::Label::new(None),
            gtk::Label::new(None),
        ];
        let bins = lib::Binary::get_binaries_dedup();
        let labels = bins
            .iter()
            .map(|bin| gtk::Label::new(Some(&bin.name().clone().into_string().unwrap())));
        for (i, label) in labels.enumerate() {
            if i < 5 {
                text_boxes[i] = label;
            }
        }
        AppSearchResults {
            text_boxes: text_boxes,
        }
    }
    pub fn widgets(&self) -> &[gtk::Label; 5] {
        &self.text_boxes
    }
    pub fn search(&mut self, query: &str) -> &[gtk::Label] {
        let mut bins = lib::Binary::get_binaries_dedup();
        for item in self.text_boxes.iter_mut() {
            for bin in bins.iter_mut() {
                let binName = bin.name().clone().into_string().unwrap_or_else(|e| {
                    eprintln!("Error:{:?}", e);
                    std::process::exit(1);
                });
                if binName.contains(query) {
                    eprintln!("{}", binName);
                    item.set_text(&binName);
                }
            }
        }
        &self.text_boxes
    }
}

//TODO: Make this use grid and add labels returned from AppSearchResults.
fn build_ui(app: &gtk::Application) {
    let window = gtk::ApplicationWindowBuilder::new()
        .application(app)
        .title("Launcher")
        .window_position(gtk::WindowPosition::Center)
        .default_height(400)
        .default_width(600)
        .resizable(false)
        .build();
    let entry = gtk::Entry::new();
    let search = std::rc::Rc::new(std::cell::RefCell::new(AppSearchResults::new("")));
    let search_clone = search.clone();
    entry.connect_changed(move |entry| {
        let query = match entry.get_text() {
            Some(T) => T.as_str().to_owned(),
            None => "".to_owned(),
        };
        eprintln!("{:?}", &query);
        let items = search_clone.borrow_mut().search(&query).to_owned();
        items.iter().for_each(|x| eprintln!("{:?}", x.get_text()))
    });
    window.add(&entry);
    window.show_all();
}
fn main() {
    let app = gtk::Application::new(None, gio::ApplicationFlags::empty()).unwrap_or_else(|err| {
        eprintln!("Could not access gtk,{}.", err);
        std::process::exit(1);
    });
    app.connect_activate(|app| build_ui(app));
    app.run(&std::env::args().collect::<Vec<_>>());
    let bins = lib::Binary::get_binaries_dedup();
}
