use rltk::{VirtualKeyCode, Rltk,Point};
use specs::prelude::*;

use super::{Position,Player,TileType,Map,State,Viewshed,RunState};
use std::cmp::{min, max};

pub fn try_move_player(delta_x:i32,delta_y:i32,ecs:&mut World)
{
        let mut positions = ecs.write_storage::<Position>();
        let mut players = ecs.write_storage::<Player>();
        let mut viewsheds = ecs.write_storage::<Viewshed>();

        let map = ecs.fetch::<Map>();

        for (_player, pos, viewshed) in (&mut players, &mut positions, &mut viewsheds).join() 
            {
            let new_x=  min(79,max(0,pos.x+delta_x));
            let new_y = min(49,max(0,pos.y+delta_y));
            let destination_idx = map.xy_idx(new_x,new_y);
            if !map.blocked[destination_idx]
            {
                pos.x = new_x;
                pos.y = new_y;
                let mut ppos = ecs.write_resource::<Point>();
                ppos.x = pos.x;
                ppos.y = pos.y;
                viewshed.dirty = true;
            }
        }

}
pub fn player_input(gs:&mut State,ctx:&mut Rltk) -> RunState
{
   // Player movement
   match ctx.key {
    None => { return RunState::Paused } // Nothing happened
    Some(key) => match key {
        VirtualKeyCode::Left |
        VirtualKeyCode::Numpad4 |
        VirtualKeyCode::H => try_move_player(-1, 0, &mut gs.ecs),

        VirtualKeyCode::Right |
        VirtualKeyCode::Numpad6 |
        VirtualKeyCode::L => try_move_player(1, 0, &mut gs.ecs),

        VirtualKeyCode::Up |
        VirtualKeyCode::Numpad8 |
        VirtualKeyCode::K => try_move_player(0, -1, &mut gs.ecs),

        VirtualKeyCode::Down |
        VirtualKeyCode::Numpad2 |
        VirtualKeyCode::J => try_move_player(0, 1, &mut gs.ecs),

        // Diagonals
        VirtualKeyCode::Numpad9 |
        VirtualKeyCode::Y => try_move_player(1, -1, &mut gs.ecs),

        VirtualKeyCode::Numpad7 |
        VirtualKeyCode::U => try_move_player(-1, -1, &mut gs.ecs),

        VirtualKeyCode::Numpad3 |
        VirtualKeyCode::N => try_move_player(1, 1, &mut gs.ecs),

        VirtualKeyCode::Numpad1 |
        VirtualKeyCode::B => try_move_player(-1, 1, &mut gs.ecs),

        _ => { return RunState::Paused }
    },
}
RunState::Running
}