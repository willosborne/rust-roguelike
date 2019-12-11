use crate::consts::*;
use crate::types::*;
use crate::utils::mut_two;

use tcod::colors;


pub fn player_move_or_attack(dx: i32, dy: i32, game: &mut Game, objects: &mut [Object]) {
    let x = objects[PLAYER].x + dx;
    let y = objects[PLAYER].y + dy;

    // try to find an attackable objects
    let target_id = objects
        .iter()
        .position(|object| object.fighter.is_some() && object.pos() == (x, y));

    match target_id {
        Some(target_id) => {
            let (player, target) = mut_two(PLAYER, target_id, objects);
            player.attack(target, game);
        },
        None => {
            move_by(PLAYER, dx, dy, &game.map, objects);
        }
    }
}

pub fn player_death(player: &mut Object, game: &mut Game) {
    game.messages.add("You died!", colors::RED);

    player.char = '%';
    player.color = colors::DARK_RED;
}

pub fn monster_death(monster: &mut Object, game: &mut Game) {
    game.messages.add(format!("The {} dies!", monster.name), colors::ORANGE);
    monster.char = '%';
    monster.color = colors::DARK_RED;
    monster.blocks = false;
    monster.alive = false;
    monster.fighter = None;
    monster.ai = None;
    monster.name = format!("{} corpse", monster.name);
}

pub fn pick_item_up(object_id: usize, game: &mut Game, objects: &mut Vec<Object>) {
    if game.inventory.len() > 26 {
        game.messages.add(
            format!("Your inventory is full. Cannot pick up {}.", objects[object_id].name),
            colors::RED,
        );
    } else {
        let item = objects.swap_remove(object_id);
        game.messages.add(format!("Picked up {}", item.name), colors::GREEN);
        game.inventory.push(item);
    }
}

pub fn use_item(inventory_id: usize, tcod: &mut Tcod, game: &mut Game, objects: &mut [Object]) {
    use Item::*;

    if let Some(item) = game.inventory[inventory_id].item {
        let on_use = match item {
            Heal => cast_heal
        };

        match on_use(inventory_id, tcod, game, objects) {
            UseResult::UsedUp => {
                game.inventory.remove(inventory_id);
            }
            UseResult::Cancelled => {
                game.messages.add("Cancelled", colors::WHITE);
            }
        }
    } else {
        game.messages.add(
            format!("The {} cannot be used.", game.inventory[inventory_id].name),
            colors::RED,
        );
    }
}

fn cast_heal(inventory_id: usize, tcod: &mut Tcod, game: &mut Game, objects: &mut [Object]) -> UseResult {
    if let Some(fighter) = objects[PLAYER].fighter {
        if fighter.hp == fighter.max_hp {
            game.messages.add("You are already at maximum health.", colors::RED);
            return UseResult::Cancelled;
        }
        game.messages
            .add("Your wounds recede as the potion courses through your veins.", colors::GREEN);
        objects[PLAYER].heal(HEAL_AMOUNT);
        return UseResult::UsedUp;
    }
    UseResult::Cancelled
}
