use crate::consts::*;
use crate::types::*;
use crate::map::*;


pub fn player_move_or_attack(dx: i32, dy: i32, game: &Game, objects: &mut [Object]) {
    let x = objects[PLAYER].x + dx;
    let y = objects[PLAYER].y + dy;

    // try to find an attackable objects
    let target_id = objects
        .iter()
        .position(|object| object.pos() == (x, y));

    match target_id {
        Some(target_id) => {
            println!("Your fists bounce harmlessly off the {}'s hide.", objects[target_id].name);
        },
        None => {
            move_by(PLAYER, dx, dy, &game.map, objects);
        }
    }
}
