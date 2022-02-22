use crate::prelude::*;

#[system]
#[read_component(Point)]
#[read_component(Player)]
#[read_component(Enemy)]
pub fn collisions(world: &SubWorld, commands: &mut CommandBuffer) {
    let mut players = <&Point>::query().filter(component::<Player>());
    let mut enemies = <(Entity, &Point)>::query().filter(component::<Enemy>());

    let player_pos = players.iter(world).next().expect("Get player");

    enemies
        .iter(world)
        .filter(|(_, pos)| **pos == *player_pos)
        .for_each(|(entity, _)| {
            commands.remove(*entity);
        });
}
