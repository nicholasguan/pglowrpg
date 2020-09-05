use crate::layers::river_mapping::*;
use crate::worldgen;

pub fn distance(rg: &RgParams) -> usize {
	((rg.dv.x0 as i32 - rg.dv.x1 as i32).abs()
		+ (rg.dv.y0 as i32 - rg.dv.y1 as i32).abs()) as usize
}

pub fn length(rg: &RgParams) -> usize {
	((rg.dv.x0 as f32 - rg.dv.x1 as f32).powf(2.0)
		+ (rg.dv.y0 as f32 - rg.dv.y1 as f32).powf(2.0))
	.powf(0.5) as usize
}

pub fn vector_within_len(
	rg: &RgParams,
	allowed: usize,
) -> bool {
	length(rg) <= allowed
}

pub fn vector_bound(
	rg: &mut RgParams,
	allowed: usize,
) {
	//bound up
	if rg.dv.x0 >= allowed {
		rg.dv.x0 = allowed - 1;
	}
	if rg.dv.y0 >= allowed {
		rg.dv.y0 = allowed - 1;
	}
	if rg.dv.x1 >= allowed {
		rg.dv.x1 = allowed - 1;
	}
	if rg.dv.y1 >= allowed {
		rg.dv.y1 = allowed - 1;
	}

	//bound down
	if rg.dv.x0 == 0 {
		rg.dv.x0 = 0;
	}
	if rg.dv.y0 == 0 {
		rg.dv.y0 = 0;
	}
	if rg.dv.x1 == 0 {
		rg.dv.x1 = 0;
	}
	if rg.dv.y1 == 0 {
		rg.dv.y1 = 0;
	}
}

//doesn't work since mapping is later maybe sort out on
//intersections by dropping the paths that start too close?
pub fn source_at_distance(
	i: usize,
	j: usize,
	rg: &mut RgParams,
	lp: &worldgen::LayerPack,
	allowed: usize,
) -> bool {
	let mut condition = true;
	let x_min = i.saturating_sub(allowed / 2);
	let y_min = j.saturating_sub(allowed / 2);
	let mut x_max = i + allowed / 2;
	let mut y_max = j + allowed / 2;

	if x_max > lp.wi.map_size {
		x_max = lp.wi.map_size;
	}

	if y_max > lp.wi.map_size {
		y_max = lp.wi.map_size;
	}

	for x in x_min..x_max {
		for y in y_min..y_max {
			let index = rg.xy.ind(x, y);

			if river_mask_get(rg.river_mask_map[index]) != NO_RIVER {
				condition = false;
				break;
			}
		}
		if !condition {
			break;
		}
	}
	//println!("{:?}", condition);
	condition
}

pub fn vector_start(
	rg: &mut RgParams,
	i: usize,
	j: usize,
) {
	rg.dv.x0 = i;
	rg.dv.y0 = j;
}

pub fn vector_end(
	rg: &mut RgParams,
	lp: &mut worldgen::LayerPack,
) {
	let mut water_bodies = false;

	for cell_v in rg.wmask_map.iter() {
		if *cell_v >= lp.wi.river_attr_pool_size_pow {
			water_bodies = true;
		}
	}

	if water_bodies {
		with_water(rg, lp);
	} else {
		without_water(rg, lp);
	}
}

pub fn vector_end_stream(
	rg: &mut RgParams,
	lp: &mut worldgen::LayerPack,
) {
	rg.dv.x1 = lp.wi.map_size / 2;
	rg.dv.y1 = lp.wi.map_size / 2;
}