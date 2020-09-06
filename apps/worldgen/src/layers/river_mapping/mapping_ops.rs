//use crate::array_ops::noise_maps::point_multi;
use crate::layers::river_mapping::*;
//use crate::worldgen;
use constants::world_constants::*;
//use prng::prng;

use std::cmp::Ordering;

//▒▒▒▒▒▒▒▒▒▒▒ MAP PATHS ▒▒▒▒▒▒▒▒▒▒▒
pub fn map_paths(
	rg: &mut RgParams,
	lp: &mut worldgen::LayerPack,
) {
	for river_entry in rg.rivers_paths.list.clone().iter().rev() {
		map_rivers_reverse(rg, lp, river_entry);
	}
}

//▒▒▒▒▒▒▒▒▒▒ RIVER MAPPING ▒▒▒▒▒▒▒▒▒▒▒
fn map_rivers_reverse(
	rg: &mut RgParams,
	lp: &mut worldgen::LayerPack,
	river_entry: &RiverEntry,
) {
	//Aliases
	//Maps masks
	let m_terrain = lp.topography.masks.terrain;
	let m_watermask = lp.topography.masks.watermask;
	let m_river_elem = lp.rivers.masks.element;
	let m_upstream = lp.rivers.masks.upstream;
	let m_downstream = lp.rivers.masks.downstream;
	let m_temp = lp.climate.masks.temperature;

	let path_array = &river_entry.path_array;

	let i_source = path_array[0].0;
	let j_source = path_array[0].1;
	let index_source = rg.xy.ind(i_source, j_source);
	let index_river_source =
		rg.xy.ind(rg.river_source.0, rg.river_source.1);
	let index_end = rg.xy.ind(rg.river_end.0, rg.river_end.1);

	let terrain_source = lp.topography.read(m_terrain, index_source);

	//Store temporary values
	rg.river_id = river_entry.river_id;
	rg.river_width = river_entry.width;
	rg.river_source = river_entry.source;
	rg.river_end = river_entry.end;

	let mut river_length = 0;

	//Cold run to figure out river lengths and truncate short ones
	for n in path_array.windows(2) {
		
		//Must be here
		river_length += 1;

		//Aliases
		let i0 = n[0].0;
		let j0 = n[0].1;
		let i1 = n[1].0;
		let j1 = n[1].1;

		let index_current = rg.xy.ind(i0, j0);
		let index_downstr = rg.xy.ind(i1, j1);

		let temp_current = lp.climate.read(m_temp, index_current);
		let river_elem_current =
			lp.rivers.read(m_river_elem, index_current);
		let river_elem_downstr =
			lp.rivers.read(m_river_elem, index_downstr);
		let river_id_downstr = lp.rivers_id.read(index_downstr);
		let wmask_current =
			lp.topography.read(m_watermask, index_current);
		
		
		//Stop if the temperature is too low
		let temp = translate::get_abs(
			temp_current as f32,
			255.0,
			lp.wi.abs_temp_min as f32,
			lp.wi.abs_temp_max as f32,
		) as isize;

		if temp <= TEMP_POLAR {
			break;
		}

		//Minimum sink size check and stop if sink is big enough
		if wmask_current > lp.wi.river_sink_min_pool_size_pow {
			break;
		}

		//Check if river spawns on an existing one
		//also terminates rivers upon crossing
		//but allows loops, which should be adressed below
		//enabling makes rivers disappear sometimes
		if (river_elem_current != NO_RIVER) && (river_length == 1) {
			break;
		}
		
		match river_elem_downstr {
			NO_RIVER => {}
			RIVER_BODY => {
				if rg.river_id != river_id_downstr {
					break;
				}
			}
			RIVER_WATERFALL => {
				if rg.river_id != river_id_downstr {
					break;
				}
			}
			RIVER_WATERFALLS_MUL => {
				if rg.river_id != river_id_downstr {
					break;
				}
			}
			RIVER_SOURCE => {
				if rg.river_id != river_id_downstr {
					break;
				}
			}
			RIVER_END => {}
			_ => {
				println!("Elem downstream: {:?}", river_elem_downstr);
				println!("{:?}", lp.rivers.expose(index_downstr));
				panic!("Unexpected river element type value");
			}
		}
	}

	if river_length <= lp.wi.river_min_length {
		return;
	}

	//Reset
	let mut river_length = 0;
	//Push to the queue for future
	erosion_initiate(rg, lp, rg.river_id);

	//Main run
	for n in path_array.windows(2) {
		//Must be here
		river_length += 1;

		//Aliases
		let i0 = n[0].0;
		let j0 = n[0].1;
		let i1 = n[1].0;
		let j1 = n[1].1;

		let index_current = rg.xy.ind(i0, j0);
		let index_downstr = rg.xy.ind(i1, j1);

		let temp_current = lp.climate.read(m_temp, index_current);
		let river_elem_current =
			lp.rivers.read(m_river_elem, index_current);
		let river_elem_downstr =
			lp.rivers.read(m_river_elem, index_downstr);
		let river_id_downstr = lp.rivers_id.read(index_downstr);
		let wmask_current =
			lp.topography.read(m_watermask, index_current);
		let wmask_downstr =
			lp.topography.read(m_watermask, index_downstr);
		let terrain_current =
			lp.topography.read(m_terrain, index_current);
		let terrain_downstr =
			lp.topography.read(m_terrain, index_downstr);

		//Stop if the temperature is too low
		let temp = translate::get_abs(
			temp_current as f32,
			255.0,
			lp.wi.abs_temp_min as f32,
			lp.wi.abs_temp_max as f32,
		) as isize;

		if temp <= TEMP_POLAR {
			break;
		}

		//Minimum sink size check and stop if sink is big enough
		if wmask_current > lp.wi.river_sink_min_pool_size_pow {
			break;
		}

		//Check if river spawns on an existing one
		//also terminates rivers upon crossing
		//but allows loops, which should be adressed below
		//enabling makes rivers disappear sometimes
		if (river_elem_current != NO_RIVER) && (river_length == 1) {
			break;
		}

		//Mark upstream neighbor
		//skip first cell, which is source
		if river_length > 1 {
			let i0_prev = rg.upstream_neighbor.0;
			let j0_prev = rg.upstream_neighbor.1;

			let upstream_neighbor =
				neighbor_flag(i0, j0, i0_prev, j0_prev);
			lp.rivers.write(
				upstream_neighbor,
				m_upstream,
				index_current,
			);
		}

		//Remember for next step
		rg.upstream_neighbor = (i0, j0);

		//map according to mask
		match river_elem_downstr {
			NO_RIVER => {
				sort_uninterrupted(
					rg,
					lp,
					index_current,
					index_river_source,
				);

				//Write down downstream neighbor direction
				let downstream_neighbor =
					neighbor_flag(i0, j0, i1, j1);
				lp.rivers.write(
					downstream_neighbor,
					m_downstream,
					index_current,
				);

				//If the end of the river is on land (map border)
				//then make sure the last cell is marked
				if (wmask_downstr == NO_WATER)
					&& (index_downstr == index_end)
				{
					lp.rivers.write(
						RIVER_BODY,
						m_river_elem,
						index_downstr,
					);
				}
			}

			RIVER_BODY => {
				sort_crossing(
					rg,
					lp,
					index_current,
					index_river_source,
					index_downstr,
				);

				//Make a waterfall if elevation differs
				if terrain_downstr < terrain_current {
					lp.rivers.write(
						RIVER_WATERFALL,
						m_river_elem,
						index_downstr,
					);
				}

				//Break river if it is not a loop, let loops go on
				if rg.river_id != river_id_downstr {
					break;
				}
			}

			RIVER_WATERFALL => {
				sort_crossing(
					rg,
					lp,
					index_current,
					index_river_source,
					index_downstr,
				);

				//Make a waterfalls if elevation differs and waterfall exist
				if terrain_downstr < terrain_current {
					lp.rivers.write(
						RIVER_WATERFALLS_MUL,
						m_river_elem,
						index_downstr,
					);
				}

				//Break river if it is not a loop, let loops go on
				if rg.river_id != river_id_downstr {
					break;
				}
			}

			RIVER_WATERFALLS_MUL => {
				sort_crossing(
					rg,
					lp,
					index_current,
					index_river_source,
					index_downstr,
				);

				//Break river if it is not a loop, let loops go on
				if rg.river_id != river_id_downstr {
					break;
				}
			}

			RIVER_SOURCE => {
				sort_crossing(
					rg,
					lp,
					index_current,
					index_river_source,
					index_downstr,
				);

				//Break river if it is not a loop, let loops go on
				if rg.river_id != river_id_downstr {
					break;
				}
			}

			RIVER_END => {
				sort_uninterrupted(
					rg,
					lp,
					index_current,
					index_river_source,
				);

				//Write down downstream neighbor direction
				let downstream_neighbor =
					neighbor_flag(i0, j0, i1, j1);
				lp.rivers.write(
					downstream_neighbor,
					m_downstream,
					index_current,
				);
			}

			_ => {
				println!("Elem downstream: {:?}", river_elem_downstr);
				println!("{:?}", lp.rivers.expose(index_downstr));
				panic!("Unexpected river mask value");
			}
		} //match
	}
}

//▒▒▒▒▒▒▒▒▒ SORT INTERSECTIONS ▒▒▒▒▒▒▒▒▒
//UNINTERRUPTED
fn sort_uninterrupted(
	rg: &mut RgParams,
	lp: &mut worldgen::LayerPack,
	index_current: usize,
	index_source: usize,
) {
	//Aliases
	let m_terrain = lp.topography.masks.terrain;
	let m_watermask = lp.topography.masks.watermask;
	let m_river_elem = lp.rivers.masks.element;
	let m_river_width = lp.rivers.masks.width;
	let m_upstream = lp.rivers.masks.upstream;
	let m_downstream = lp.rivers.masks.downstream;
	let m_temp = lp.climate.masks.temperature;

	lp.rivers.write(RIVER_BODY, m_river_elem, index_current);
	lp.rivers.write(RIVER_SOURCE, m_river_elem, index_source);

	lp.rivers_id.write(rg.river_id, index_current);

	lp.rivers
		.write(rg.river_width, m_river_width, index_current);
}

//CROSSING
fn sort_crossing(
	rg: &mut RgParams,
	lp: &mut worldgen::LayerPack,
	index_current: usize,
	index_source: usize,
	index_downstr: usize,
) {
	//Aliases
	let m_terrain = lp.topography.masks.terrain;
	let m_watermask = lp.topography.masks.watermask;
	let m_river_elem = lp.rivers.masks.element;
	let m_river_width = lp.rivers.masks.width;
	let m_upstream = lp.rivers.masks.upstream;
	let m_downstream = lp.rivers.masks.downstream;
	let m_temp = lp.climate.masks.temperature;

	lp.rivers.write(RIVER_BODY, m_river_elem, index_current);
	lp.rivers.write(RIVER_SOURCE, m_river_elem, index_source);

	lp.rivers_id.write(rg.river_id, index_current);

	lp.rivers
		.write(rg.river_width, m_river_width, index_current);

	//Modify river downstream
	width_routine(rg, lp, index_current, index_downstr);
	erosion_adjust(rg, lp, index_current, index_downstr);
}

//▒▒▒▒▒▒▒▒▒▒▒ ROUTINES ▒▒▒▒▒▒▒▒▒▒▒▒
//WIDTH ROUTINE
fn width_routine(
	rg: &mut RgParams,
	lp: &mut worldgen::LayerPack,
	_index_current: usize,
	index_downstr: usize,
) {
	//Aliases
	let m_river_width = lp.rivers.masks.width;

	let river_id_downstr = lp.rivers_id.read(index_downstr);
	let river_width_downstr =
		lp.rivers.read(m_river_width, index_downstr);

	//Find the downstream river in queue and its width
	let result =
		rg.rivers_paths.width_queue.iter().rev().by_ref().find(
			|WidthEntry {
			     river_id_downstr, ..
			 }| { river_id_downstr == river_id_downstr },
		);

	//Get the width value from river downstream
	let width_downstr = match result {
		Some(x) => {
			//If in queue - return its last recorded value
			x.width_new
		}
		None => {
			//If not in queue - just take its width as is
			river_width_downstr
		}
	};

	//Increment the width downstream
	let mut width_downstr_new = width_downstr.saturating_add(1);

	//Bound upper value by 12 order
	if width_downstr_new > RIVER_WIDTH_ORDER_MAX {
		width_downstr_new = RIVER_WIDTH_ORDER_MAX;
	}

	//Store new value for future
	rg.rivers_paths.width_queue.push(WidthEntry {
		river_id_downstr,
		width_new: width_downstr_new,
	});
}

//EROSION ROUTINES
fn erosion_adjust(
	rg: &mut RgParams,
	lp: &mut worldgen::LayerPack,
	index_current: usize,
	index_downstr: usize,
) {
	//Aliases
	let m_river_width = lp.rivers.masks.width;
	let m_terrain = lp.topography.masks.terrain;

	let river_id_downstr = lp.rivers_id.read(index_downstr);
	let terrain_current =
		lp.topography.read(m_terrain, index_current);
	let terrain_downstr =
		lp.topography.read(m_terrain, index_downstr);

	//Add difference in topography to queue
	let terrain_diff = match terrain_downstr.cmp(&terrain_current) {
		Ordering::Greater => terrain_downstr - terrain_current,
		Ordering::Equal => 0,
		Ordering::Less => return,
	};

	//Store for future use
	rg.rivers_paths.erosion_queue.push(ErosionEntry {
		river_id_downstr,
		terrain_diff,
	});
}

fn erosion_initiate(
	rg: &mut RgParams,
	lp: &mut worldgen::LayerPack,
	river_id: u16,
) {
	rg.rivers_paths.erosion_queue.push(ErosionEntry {
		river_id_downstr: river_id,
		terrain_diff: 0,
	});
}

fn neighbor_flag(
	i0: usize,
	j0: usize,
	i1: usize,
	j1: usize,
) -> u16 {
	let di: isize = i1 as isize - i0 as isize;
	let dj: isize = j1 as isize - j0 as isize;

	let neighbor = match (di, dj) {
		(0, 0) => panic!("neighbor downstream matches current"),
		(1, 0) => 0,   //N
		(1, 1) => 1,   //NE
		(0, 1) => 2,   //E
		(-1, 1) => 3,  //SE
		(-1, 0) => 4,  //S
		(-1, -1) => 5, //SW
		(0, -1) => 6,  //W
		(1, -1) => 7,  //NW

		(_, _) => panic!("unexpected neighbor {:?}", (di, dj)),
	};
	neighbor
}
