use crate::prelude::*;

#[system]
pub fn transition_turn(#[resource] turn: &mut TurnState) {
    let next_turn = match turn {
        TurnState::AwaitingInput => return,
        TurnState::PlayerTurn => TurnState::MonsterTurn,
        TurnState::MonsterTurn => TurnState::AwaitingInput,
    };
    *turn = next_turn;
}
