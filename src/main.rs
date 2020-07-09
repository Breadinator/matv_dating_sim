use rand::Rng;
use std::io;

static NAMES: [&str; 8] = [
	"Sam",
	"Producer Jeff",
	"Greg",
	"Boombot",
	"Gary",
	"Anne",
	"Raze",
	"Ash",
];

static HAIRS: [&str; 3] = [
    "Blonde",
    "Brunette",
    "Redhead",
];

static GENDERS: [&str; 3] = [
    "Man",
    "Woman",
    "Non-binary",
];

static LOCATIONS: [&str; 3] = [
    "Pub",
    "Beach",
    "Party",
];

fn main() {
	pub struct Date<'a> {
		name: &'a str,
		hair: &'a str,
		gender: &'a str,
		location: &'a str,
	}

	impl Date<'_> {
		pub fn new(&mut self) {
			self.name = get_rand_name();
			self.hair = get_rand_hair();
			self.gender = get_rand_gender();
			self.location = get_rand_location();
		}
	}

	let mut date = Date {name: "", hair: "", gender: "", location: ""};

	loop {
		date.new();

		println!("#####################################\n#### A NEW CHALLENGER APPROACHES ####");
		println!(
			"     Name: {}\n     Gender: {}\n     Hair colour: {}\n     Location: {}\n",
			date.name,
			date.gender,
			date.hair,
			date.location,
		);
		println!("What do you say?");

    	let mut response = String::new();
    	io::stdin()
	    	.read_line(&mut response)
	    	.expect("Failed to read line ;~;");
	    let response = response.trim();

	    if response == "quit" {
	    	break;
	    } else if response == "Are you matt andrews? Cuz you look really good LUL" {
	    	println!("{}", get_pos_resp());
	    } else {
	    	println!("{}", get_neg_resp());
	    }
	    println!("\n");
	}
}

fn get_rand_name() -> &'static str {
	let i = rand::thread_rng().gen_range(0, NAMES.len());
	NAMES[i]
}

fn get_rand_hair() -> &'static str {
	let i = rand::thread_rng().gen_range(0, HAIRS.len());
	HAIRS[i]
}

fn get_rand_gender() -> &'static str {
	let i = rand::thread_rng().gen_range(0, GENDERS.len());
	GENDERS[i]
}

fn get_rand_location() -> &'static str {
	let i = rand::thread_rng().gen_range(0, LOCATIONS.len());
	LOCATIONS[i]
}

fn get_pos_resp() -> &'static str {
	let resp = [
		"They don't understand the line, but they're enamoured by your charisma and propose right there.",
		"PogChamp they gave you their number!",
		"Let's go, they added you on uplay."
	];
	resp[rand::thread_rng().gen_range(0, resp.len())]
}

fn get_neg_resp() -> &'static str {
	let resp = [
		"They look slightly disgusted, and decline.",
		"Lol, no.",
		"I have a boyfriend :)"
	];
	resp[rand::thread_rng().gen_range(0, resp.len())]
}