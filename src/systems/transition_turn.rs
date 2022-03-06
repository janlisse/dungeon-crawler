use crate::prelude::*;

#[system]
#[read_component(Health)]
#[read_component(Point)]
#[read_component(Player)]
#[read_component(AmuletOfYala)]
pub fn transition_turn(world: &SubWorld, #[resource] turn: &mut TurnState) {
    let mut player_hp = <&Health>::query().filter(component::<Player>());
    let mut amulet = <&Point>::query().filter(component::<AmuletOfYala>());
    let mut player = <&Point>::query().filter(component::<Player>());
    let amulet_pos = amulet.iter(world).nth(0).unwrap();
    let player_pos = player.iter(world).nth(0).unwrap();
    let current_state = turn.clone();
    let mut new_state = match turn {
        TurnState::AwaitingInput => return,
        TurnState::PlayerTurn => TurnState::MonsterTurn,
        TurnState::MonsterTurn => TurnState::AwaitingInput,
        _ => current_state,
    };

    player_hp.iter(world).for_each(|hp| {
        if hp.current < 1 {
            new_state = TurnState::GameOver;
        }
        if player_pos == amulet_pos {
            new_state = TurnState::Victory;
        }
    });
    *turn = new_state;
}
