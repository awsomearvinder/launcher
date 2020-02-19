#![feature(drain_filter, const_in_array_repeat_expressions, maybe_uninit_extra)]
use gio;
use gio::prelude::*;
use gtk::prelude::*;
use gtk::EntryExt;
use gtk::GridExt;
use gtk::LabelExt;

mod lib;
const NUM_WIDGETS: usize = 5;

#[derive(Debug, Clone)]
struct AppSearchResults {
    binary_results: [lib::Binary; NUM_WIDGETS],
}

impl AppSearchResults {
    pub fn new(search_query: &str) -> Self {
        AppSearchResults {
            binary_results: Self::get_binaries(search_query),
        }
    }

    pub fn widgets(&self) -> [gtk::Label; NUM_WIDGETS] {
        let mut labels: [std::mem::MaybeUninit<gtk::Label>; NUM_WIDGETS] =
            unsafe { std::mem::MaybeUninit::uninit().assume_init() };

        for i in 0..NUM_WIDGETS {
            labels[i].write(::gtk::Label::new(None));
        }

        for (i, item) in self.binary_results.iter().enumerate() {
            labels[i].write(gtk::Label::new(Some(
                &item.name().clone().into_string().unwrap_or_else(|e| {
                    eprintln!("Error:  {:?}, is bin name a valid utf-8?", e);
                    std::process::exit(1);
                }),
            )));
        }
        unsafe { std::mem::transmute::<_, [gtk::Label; NUM_WIDGETS]>(labels) }
    }

    pub fn get_binaries(query: &str) -> [lib::Binary; 5] {
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
                    eprintln!("Error: {:?}, is bin name utf-8?", e);
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
    pub fn search(&mut self, query: &str) {
        self.binary_results = Self::get_binaries(query);
    }
}

fn load_css() -> std::io::Result<()> {
    let provider = gtk::CssProvider::new();
    provider
        .load_from_data(
            std::fs::read_to_string(std::path::Path::new("/home/bender/.config/sway/style.css"))?
                .as_bytes(),
        )
        .expect("Failed to load CSS");
    // We give the CssProvided to the default screen so the CSS rules we added
    // can be applied to our window.
    gtk::StyleContext::add_provider_for_screen(
        &gdk::Screen::get_default().expect("Error initializing gtk css provider."),
        &provider,
        gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
    );
    Ok(())
}

fn build_ui(app: &gtk::Application) {
    let window = gtk::ApplicationWindowBuilder::new()
        .application(app)
        .title("Launcher")
        .window_position(gtk::WindowPosition::Center)
        .default_height(320)
        .default_width(600)
        .resizable(false)
        .build();
    let gtk_grid = gtk::Grid::new();
    let search = std::rc::Rc::new(std::cell::RefCell::new(AppSearchResults::new("")));
    let entry = gtk::Entry::new();
    let widgets = std::rc::Rc::new(std::cell::RefCell::new(search.borrow().widgets()));
    gtk_grid.set_column_homogeneous(true);
    gtk_grid.set_row_homogeneous(true);
    gtk_grid.set_row_spacing(0);
    gtk_grid.set_cell_height(&entry, 1);
    gtk_grid.attach(&entry, 0, 0, 1, 3);
    widgets.borrow().iter().enumerate().for_each(|(i, item)| {
        item.set_halign(gtk::Align::Start);
        gtk_grid.attach(item, 0, (i as i32 + 1) * 3, 1, 3);
    });
    let search_clone = search.clone();
    let widgets_clone = widgets.clone();
    entry.connect_changed(move |entry| {
        let query = match entry.get_text() {
            Some(input) => input.as_str().to_owned(),
            None => "".to_owned(),
        };
        search_clone.borrow_mut().search(&query);
        let mut binaries = search_clone.borrow().binaries().clone();
        let mut binary_iter = binaries.iter_mut();
        widgets_clone.borrow_mut().iter_mut().for_each(|widget| {
            let name = match binary_iter.next() {
                Some(binary) => binary.name().clone().into_string().unwrap(),
                None => String::from(""),
            };
            widget.set_text(&name);
        });
    });
    let search_clone2 = search.clone();
    entry.connect_activate(move |_entry| {
        search_clone2.borrow().binaries()[0].run().unwrap();
        std::process::exit(0);
    });
    window.add(&gtk_grid);
    window.show_all();
}
fn main() {
    let app = gtk::Application::new(None, gio::ApplicationFlags::empty()).unwrap_or_else(|err| {
        eprintln!("Could not access gtk,{}.", err);
        std::process::exit(1);
    });
    app.connect_activate(|app| {
        load_css().unwrap_or_else(|e| {
            eprintln!("Couldn't load style.css {}", e);
            std::process::exit(1);
        });
        build_ui(app);
    });
    app.run(&std::env::args().collect::<Vec<_>>());
}
