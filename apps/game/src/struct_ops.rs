use crate::*;

//Common shared data for all the functions
pub struct GameData {
	//World
	pub lp: LayerPack,
	pub options: options::Stuff,
	//Commands
	pub commands: strings::commands::Stuff,
	pub commands_vec: Vec<String>,
	//Coordinates
	//+x is north, +y is east due to how worldgen was made
	pub x: usize,
	pub y: usize,
	pub index: usize,
	//Temporary working variables
	//World data abs values
	pub temp: isize,
	pub rain: usize,
	pub elev: usize,
	pub water: u16,
	pub biome: u8,
	pub georeg_id: u16,
	pub river_id: u16,
	pub river_width: u16,
	pub river_element: u16,
	pub river_upstream: u16,
	pub river_downstream: u16,
	//Other in-game variables
	pub map_render_size: usize,
}

pub struct GameStrings {
	//Structs
	pub gm_str: strings::game_strings::Stuff,
	pub panic_str: strings::panic_strings::Stuff,
	pub ui_el: strings::ui_elements::Stuff,
	//Temporary value
	pub s: String,
	//Basic strings
	pub coord_str: String,
	pub temp_str: String,
	pub biome_str: String,
	pub georeg_id_str: String,
	pub rain_str: String,
	pub elev_str: String,
	pub water_str: String,
	pub river_str: String,
}

//▒▒▒▒▒▒▒▒▒▒▒▒ INITS ▒▒▒▒▒▒▒▒▒▒▒▒▒
pub fn init_gd(
	gs: &GameStrings,
	options: options::Stuff,
	commands: strings::commands::Stuff,
) -> Option<GameData> {
	let lp = match get_layerpack(&gs) {
		Some(lp) => lp,
		_ => return None,
	};
	let gd = GameData {
		//Move previously cloned structs here
		lp,
		options,
		//Commands
		commands,
		commands_vec: Vec::new(),
		//Coordinates
		//+x is north, +y is east due to how worldgen was made
		x: 0,
		y: 0,
		index: 0,
		//Temporary working variables
		//World data abs values
		temp: 0,
		rain: 0,
		elev: 0,
		water: 0,
		biome: 0,
		georeg_id: 0,
		river_id: 0,
		river_width: 0,
		river_element: 0,
		river_upstream: 0,
		river_downstream: 0,
		//Other in-game variables
		map_render_size: 13,
	};
	Some(gd)
}
//Strings for printing
pub fn init_gs() -> GameStrings {
	let input_locale = options::get().locale;
	let gm_str: strings::game_strings::Stuff =
		strings::game_strings::get(&input_locale);
	let panic_str: strings::panic_strings::Stuff =
		strings::panic_strings::get(&input_locale);
	let ui_el: strings::ui_elements::Stuff =
		strings::ui_elements::get(&input_locale);
	GameStrings {
		//Structs
		gm_str,
		panic_str,
		ui_el,
		//Temporary string
		s: String::new(),
		//Basic strings
		coord_str: String::new(),
		temp_str: String::new(),
		biome_str: String::new(),
		georeg_id_str: String::new(),
		rain_str: String::new(),
		elev_str: String::new(),
		water_str: String::new(),
		river_str: String::new(),
	}
}
