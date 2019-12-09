use tcod::colors::*;
use tcod::console::*;
use tcod::input::Key;
use tcod::input::KeyCode::*;
use tcod::map::Map as FovMap;

pub mod types;
use types::*;
pub mod map;
use map::*;
pub mod consts;
use consts::*;
pub mod action;
use action::*;

use PlayerAction::*;

fn render_all(tcod: &mut Tcod, game: &mut Game, objects: &[Object], fov_recompute: bool) {
    if fov_recompute {
        let player = &objects[0];
        tcod.fov
            .compute_fov(player.x, player.y, TORCH_RADIUS, FOV_LIGHT_WALLS, FOV_ALGO);
    }

    for object in objects {
        if tcod.fov.is_in_fov(object.x, object.y) {
            object.draw(&mut tcod.con);
        }
    }

    for y in 0..MAP_HEIGHT {
        for x in 0..MAP_WIDTH {
            let wall = game.map[x as usize][y as usize].block_sight;
            let visible = tcod.fov.is_in_fov(x, y);
            let color = match (visible, wall) {
                (false, true) => COLOR_DARK_WALL,
                (true, true) => COLOR_LIGHT_WALL,
                (false, false) => COLOR_DARK_GROUND,
                (true, false) => COLOR_LIGHT_GROUND,
            };
            let explored = &mut game.map[x as usize][y as usize].explored;
            if visible {
                *explored = true;
            }
            if *explored {
                tcod.con
                    .set_char_background(x, y, color, BackgroundFlag::Set);
            }
        }
    }

    blit(
        &tcod.con,
        (0, 0),
        (SCREEN_WIDTH, SCREEN_HEIGHT),
        &mut tcod.root,
        (0, 0),
        1.0,
        1.0,
    );
}

fn main() {
    // let player = Object::new(SCREEN_WIDTH / 2, SCREEN_HEIGHT / 2, '@', WHITE);

    let mut player = Object::new(0, 0, '@', "player", BLACK, true);
    player.alive = true;
    player.fighter = Some(Fighter {
        max_hp: 30,
        hp: 30,
        defence: 2,
        attack: 5
    });

    let mut objects = vec![player];

    let mut game = Game {
        map: make_map(&mut objects),
    };

    let root = Root::initializer()
        .font("arial10x10.png", FontLayout::Tcod)
        .font_type(FontType::Greyscale)
        .size(SCREEN_WIDTH, SCREEN_HEIGHT)
        .title("rust/tcod tutorial")
        .init();

    let mut tcod = Tcod {
        root,
        con: Offscreen::new(MAP_WIDTH, MAP_HEIGHT),
        fov: FovMap::new(MAP_WIDTH, MAP_HEIGHT),
    };
    tcod::system::set_fps(LIMIT_FPS);

    for y in 0..MAP_HEIGHT {
        for x in 0..MAP_WIDTH {
            tcod.fov.set(
                x,
                y,
                !game.map[x as usize][y as usize].block_sight,
                !game.map[x as usize][y as usize].blocked,
            );
        }
    }

    let mut previous_player_position = (-1, -1);

    while !tcod.root.window_closed() {
        tcod.con.clear();

        let fov_recompute = previous_player_position != (objects[PLAYER].x, objects[PLAYER].y);
        // let fov_recompute = previous_player_position != (player.x, player.y);
        render_all(&mut tcod, &mut game, &objects, fov_recompute);

        tcod.root.flush();

        previous_player_position = objects[PLAYER].pos();

        let action = handle_keys(&mut tcod, &game, &mut objects);
        if action == Exit {
            break;
        }

        if objects[PLAYER].alive && action != DidntTakeTurn {
            for object in &objects {
                // if object isn't the player
                if (object as *const _) != (&objects[PLAYER] as *const _) {
                    // println!("The {} growls!", object.name);
                }
            }
        }
    }
}

fn handle_keys(tcod: &mut Tcod, game: &Game, objects: &mut [Object]) -> PlayerAction {
    let key = tcod.root.wait_for_keypress(true);
    let player_alive = objects[PLAYER].alive;
    match (key, key.text(), player_alive) {
        (Key {
            code: Enter,
            alt: true,
            ..
        }, _, _) => {
            let fullscreen = tcod.root.is_fullscreen();
            tcod.root.set_fullscreen(!fullscreen);
            DidntTakeTurn
        },

        (Key { code: Escape, .. }, _, _) => Exit,

        (Key { code: Up, .. }, _, true) => {
            player_move_or_attack(0, -1, game, objects);
            TookTurn
        },
        (Key { code: Down, .. }, _, true) => {
            player_move_or_attack(0, 1, game, objects);
            TookTurn
        },
        (Key { code: Left, .. }, _, true) => {
            player_move_or_attack(-1, 0, game, objects);
            TookTurn
        },
        (Key { code: Right, .. }, _, true) => {
            player_move_or_attack(1, 0, game, objects);
            TookTurn
        },

        _ => DidntTakeTurn
    }
}
