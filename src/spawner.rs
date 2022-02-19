pub use crate::prelude::*;

pub fn spawn_player(ecs: &mut World, pos: Point) {
    ecs.push((
        Player,
        pos,
        Render {
            color: ColorPair::new(WHITE, BLACK),
            glyph: to_cp437('@'),
        },
    ));
}

pub fn spawn_monster(world: &mut World, rng: &mut RandomNumberGenerator, pos: Point) {
    let glyph = match rng.range::<u16>(0, 4) {
        0 => to_cp437('E'),
        1 => to_cp437('O'),
        2 => to_cp437('o'),
        _ => to_cp437('g'),
    };
    world.push((
        Enemy,
        pos,
        Render {
            color: ColorPair::new(WHITE, BLACK),
            glyph,
        },
    ));
}
