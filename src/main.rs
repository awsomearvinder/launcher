#![feature(drain_filter)]
use gio;
use gio::prelude::*;
use gtk::prelude::*;
use gtk::EntryExt;
use gtk::GridExt;
use gtk::LabelExt;

mod lib;

#[derive(Debug, Clone)]
struct AppSearchResults {
    binary_results: [lib::Binary; 5],
}

impl AppSearchResults {
    pub fn new(search_query: &str) -> Self {
        AppSearchResults {
            binary_results: Self::search_binaries(search_query),
        }
    }
    pub fn widgets(&self) -> [gtk::Label; 5] {
        let mut labels: [gtk::Label; 5] = [
            gtk::Label::new(None),
            gtk::Label::new(None),
            gtk::Label::new(None),
            gtk::Label::new(None),
            gtk::Label::new(None),
        ];
        for (i, item) in self.binary_results.iter().enumerate() {
            labels[i] = gtk::Label::new(Some(&item.name().clone().into_string().unwrap_or_else(
                |e| {
                    eprintln!("Error:  {:?}", e);
                    std::process::exit(1);
                },
            )));
        }
        labels
    }
    fn search_binaries(query: &str) -> [lib::Binary; 5] {
        let mut binary_results: [lib::Binary; 5] = [
            lib::Binary::new(std::ffi::OsString::new(), std::path::PathBuf::new()),
            lib::Binary::new(std::ffi::OsString::new(), std::path::PathBuf::new()),
            lib::Binary::new(std::ffi::OsString::new(), std::path::PathBuf::new()),
            lib::Binary::new(std::ffi::OsString::new(), std::path::PathBuf::new()),
            lib::Binary::new(std::ffi::OsString::new(), std::path::PathBuf::new()),
        ];
        let mut bins = lib::Binary::get_binaries_dedup();
        bins.drain_filter(|item| {
            !item
                .name()
                .clone()
                .into_string()
                .unwrap_or_else(|e| {
                    eprintln!("Error: {:?}", e);
                    std::process::exit(1);
                })
                .contains(query)
        });
        for (i, label) in bins.iter().enumerate() {
            if i < 5 {
                binary_results[i] = label.clone();
            }
        }
        binary_results
    }
    pub fn binaries(&self) -> &[lib::Binary; 5] {
        &self.binary_results
    }
    pub fn search(&mut self, query: &str) -> [gtk::Label; 5] {
        self.binary_results = Self::search_binaries(query);
        self.widgets()
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
    let gtk_grid = std::rc::Rc::new(std::cell::RefCell::new(gtk::Grid::new()));
    let search = std::rc::Rc::new(std::cell::RefCell::new(AppSearchResults::new("")));
    let entry = gtk::Entry::new();
    gtk_grid.borrow_mut().add(&entry);
    let search_clone = search.clone();
    let gtk_grid_clone = gtk_grid.clone();
    entry.connect_changed(move |entry| {
        let query = match entry.get_text() {
            Some(input) => input.as_str().to_owned(),
            None => "".to_owned(),
        };
        eprintln!("{:?}", &query);
        let items = search_clone.borrow_mut().search(&query).to_owned();
        items
            .iter()
            .for_each(|item| gtk_grid_clone.borrow_mut().add(item));
        items.iter().for_each(|x| eprintln!("{:?}", x.get_text()))
    });
    let grid = &*(gtk_grid.borrow());
    window.add(grid);
    window.show_all();
}
fn main() {
    let app = gtk::Application::new(None, gio::ApplicationFlags::empty()).unwrap_or_else(|err| {
        eprintln!("Could not access gtk,{}.", err);
        std::process::exit(1);
    });
    app.connect_activate(|app| build_ui(app));
    app.run(&std::env::args().collect::<Vec<_>>());
}
