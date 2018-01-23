extern crate gtk;
extern crate gio;
extern crate rand;

use gtk::prelude::*;
use gio::prelude::*;

use std::cell::RefCell;
use std::rc::Rc;
use std::cmp::Ordering;
use rand::Rng;

// Game mode
#[derive(Debug)]
enum Mode {
	Normal,
	Hard
}

struct HeaderUi {
	headerbar: gtk::HeaderBar,
	switch: gtk::Switch,
	start_button: gtk::Button
}

impl HeaderUi {
	fn new() -> HeaderUi {
		let headerbar = gtk::HeaderBar::new();
		headerbar.set_title("Guessing Game");
		headerbar.set_show_close_button(true);

		let switch = gtk::Switch::new();
		let start_button = gtk::Button::new_with_label("Start");

		headerbar.pack_start(&switch);
		headerbar.pack_end(&start_button);

		HeaderUi { headerbar, switch, start_button }
	}
}

fn ui(app: &gtk::Application) {
	let secret: Rc<RefCell<Option<u32>>> = Rc::new(RefCell::new(None));
	let mode: Rc<RefCell<Mode>> = Rc::new(RefCell::new(Mode::Normal));
	let window = gtk::ApplicationWindow::new(app);
	let header = HeaderUi::new();
	let switch = header.switch;
	let start_button = header.start_button;

	// Window HeaderBar
	window.set_titlebar(&header.headerbar);

	// (width, height);
	window.set_default_size(600, 300);

	// (orientation, spacing)
	let container = gtk::Box::new(
		gtk::Orientation::Vertical,
		0
	);
	container.set_sensitive(false);

	// Message
	let message_box = gtk::Box::new(
		gtk::Orientation::Horizontal,
		0
	);
	message_box.set_halign(gtk::Align::Center);
	let message = gtk::Label::new("Let's play");

	// (widget, expand, fill, padding)
	// Pack message
	message_box.pack_start(
		&message,
		false,
		false,
		0
	);

	// Entry
	let entry_box = gtk::Box::new(
		gtk::Orientation::Horizontal,
		5
	);
	entry_box.set_halign(gtk::Align::Center);
	let entry = gtk::Entry::new();

	// Pack Entry
	entry_box.pack_start(
		&entry,
		false,
		false,
		0
	);

	// Guess & Stop button
	let guess_button = gtk::Button::new_with_label("Guess!");
	let stop_button = gtk::Button::new_with_label("Stop!");
	let button_box = gtk::Box::new(
		gtk::Orientation::Horizontal,
		5
	);
	button_box.pack_start(
		&guess_button,
		true,
		false,
		0
	);
	button_box.pack_start(
		&stop_button,
		true,
		false,
		0
	);

	// Add child box to container
	container.pack_start(
		&message_box,
		true,
		false,
		0
	);
	container.pack_start(
		&entry_box,
		true,
		false,
		0
	);
	container.pack_start(
		&button_box,
		true,
		false,
		0
	);

	// Add container into window
	window.add(&container);

	// And show everything
	window.show_all();

	// Handle Event Signals
	// Clone secret
	let secret_clone = secret.clone();
	// Clone mode
	let mode_clone = mode.clone();
	let mode_clone_dua = mode_clone.clone();
	// Clone headerbar
	let headerbar_clone = header.headerbar.clone();
	// Clone container
	let container_clone = container.clone();
	// Clone message
	let message_clone = message.clone();
	// Clone start_button
	let start_button_clone = start_button.clone();

	switch.connect_property_active_notify(move |switch| {
		if switch.get_active() {
			*mode_clone.borrow_mut() = Mode::Hard;
			headerbar_clone.set_title("Game Mode (1-100): HARD!");
		} else {
			*mode_clone.borrow_mut() = Mode::Normal;
			headerbar_clone.set_title("Game Mode (1-10): Normal!");
		}
	});

	start_button.connect_clicked(move |b| {
		match *mode_clone_dua.borrow() {
			Mode::Hard => {
				*secret_clone.borrow_mut() = Some(rand::thread_rng().gen_range(1, 101));
			},
			Mode::Normal => {
				*secret_clone.borrow_mut() = Some(rand::thread_rng().gen_range(1, 51));
			}
		}
		b.set_sensitive(false);
		container_clone.set_sensitive(true);
	});

	guess_button.connect_clicked(move |_| {
		let entry = entry.get_text().unwrap();
		let mut guess: Option<u32> = None;

		match entry.trim().parse() {
			Ok(num)	=> {
				guess = Some(num);
			},
			Err(_)	=> {
				message_clone.set_label("Please enter a number");
			}
		}

		if let Some(num) = guess {
			match num.cmp(&secret.borrow().unwrap()) {
				Ordering::Less	=> {
					message_clone.set_label("Too small!");
				},
				Ordering::Equal	=> {
					message_clone.set_label("You win! Congratulations");
				},
				Ordering::Greater	=> {
					message_clone.set_label("Too big!");
				}
			}
		}

	});

	stop_button.connect_clicked(move |_| {
		// We didn't use the container clone here
		container.set_sensitive(false);
		start_button_clone.set_sensitive(true);
	});
}

fn main() {
	let app = gtk::Application::new(
		"com.github.guessing_game",
		gio::ApplicationFlags::empty()
	).expect("Failed..");

	app.connect_startup(|app| {
		ui(&app);
	});

	//run app
	app.run(&std::env::args().collect::<Vec<_>>());
}
