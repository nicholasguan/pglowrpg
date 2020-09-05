use crate::array_ops::size_2_pow;
use crate::array_ops::translate;
use crate::worldgen;

use constants::world_constants::*;
use coords::Index;

pub fn get(lp: &mut worldgen::LayerPack) {
	let xy = Index {
		map_size: lp.wi.map_size,
	};

	let mut wmask_map = vec![NO_WATER; lp.layer_vec_len];

	let blank = vec![NO_WATER; lp.layer_vec_len];

	let mut ff = floodfill::FloodFill::new(&blank, lp.wi.map_size);

	//exclude dry and ice regions at once
	for i in 0..lp.wi.map_size {
		for j in 0..lp.wi.map_size {
			let index = xy.ind(i, j);

			let elev = (translate::get_abs(
				lp.topography.read(lp.topography.masks.terrain, index)
					as f32,
				255.0,
				lp.wi.abs_elev_min as f32,
				lp.wi.abs_elev_max as f32,
			) as usize);

			let temp = translate::get_abs(
				lp.climate.read(lp.climate.masks.temperature, index)
					as f32,
				255.0,
				lp.wi.abs_temp_min as f32,
				lp.wi.abs_temp_max as f32,
			) as isize;

			if temp <= TEMP_PERM_ICE {
				ff.exclusion_map[index] = true;
			}

			if elev > lp.wi.waterlevel {
				ff.exclusion_map[index] = true;
			}
		}
	}

	for i in 0..lp.wi.map_size {
		for j in 0..lp.wi.map_size {
			if !ff.exclusion_map[xy.ind(i, j)] {
				ff.map(i, j);
				write_map(lp, &mut ff);
			}
		}
	}
}

fn write_map(
	lp: &mut worldgen::LayerPack,
	ff: &mut floodfill::FloodFill<u8>,
) {
	let xy = Index {
		map_size: lp.wi.map_size,
	};

	for x in ff.x_min..=ff.x_max {
		for y in ff.y_min..=ff.y_max {
			if ff.region_map[xy.ind(x, y)] {
				let val = size_2_pow::map(ff.region_size);

				lp.topography.write(
					val as u16,
					lp.topography.masks.watermask,
					xy.ind(x as usize, y as usize),
				)
			}
		}
	}
}