use crate::prelude::*;

#[system]
#[read_component(Point)]
#[read_component(Player)]
#[read_component(Enemy)]
#[write_component(Health)]
pub fn player_input(
    world: &mut SubWorld,
    commands: &mut CommandBuffer,
    #[resource] key: &Option<VirtualKeyCode>,
    #[resource] turn_state: &mut TurnState,
) {
    if let Some(key) = key {
        let delta = match key {
            VirtualKeyCode::Left => Point::new(-1, 0),
            VirtualKeyCode::Right => Point::new(1, 0),
            VirtualKeyCode::Up => Point::new(0, -1),
            VirtualKeyCode::Down => Point::new(0, 1),
            _ => Point::new(0, 0),
        };

        let mut players = <(Entity, &Point)>::query().filter(component::<Player>());
        let (player_entity, destination) = players
            .iter(world)
            .find_map(|(entity, pos)| Some((*entity, *pos + delta)))
            .unwrap();

        let mut enemies = <(Entity, &Point)>::query().filter(component::<Enemy>());
        let mut did_something = false;

        if delta.x != 0 || delta.y != 0 {
            let maybe_hit = enemies.iter(world).find(|(_, pos)| **pos == destination);

            if let Some(enemy_hit) = maybe_hit {
                did_something = true;
                commands.push((
                    (),
                    WantsToAttack {
                        attacker: player_entity,
                        victim: *enemy_hit.0,
                    },
                ));
            } else {
                did_something = true;
                commands.push((
                    (),
                    WantsToMove {
                        entity: player_entity,
                        destination,
                    },
                ));
            }
        }
        if !did_something {
            heal(world, player_entity);
        }
        *turn_state = TurnState::PlayerTurn;
    }
}

fn heal(world: &mut SubWorld, player_entity: Entity) {
    if let Ok(mut health) = world
        .entry_mut(player_entity)
        .unwrap()
        .get_component_mut::<Health>()
    {
        health.current = i32::min(health.max, health.current + 1);
    }
}
