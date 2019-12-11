use crate::types::*;
use crate::map::*;
use crate::consts::*;
use crate::utils::mut_two;



pub fn ai_take_turn(monster_id: usize, tcod: &Tcod, game: &mut Game, objects: &mut [Object]) {
    let (m_x, m_y) = objects[monster_id].pos();

    if tcod.fov.is_in_fov(m_x, m_y) {
        if objects[monster_id].distance_to(&objects[PLAYER]) >= 2.0 {
            let (p_x, p_y) = objects[PLAYER].pos();
            move_towards(monster_id, p_x, p_y, &game.map, objects);
        } else if objects[PLAYER].fighter.map_or(false, |f| f.hp > 0) {
            let (monster, player) = mut_two(monster_id, PLAYER, objects);
            monster.attack(player, game);
        }
    }
}

fn move_towards(id: usize, target_x: i32, target_y: i32, map: &Map, objects: &mut [Object]) {
    let dx = target_x - objects[id].x;
    let dy = target_y - objects[id].y;

    let dist = ((dx.pow(2) + dy.pow(2)) as f32).sqrt();

    let dx = (dx as f32 / dist).round() as i32;
    let dy = (dy as f32 / dist).round() as i32;
    move_by(id, dx, dy, map, objects);
}
