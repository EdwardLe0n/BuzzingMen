use turbo::prelude::*;
use std::collections::VecDeque;

use crate::turbe::component::{Component, ComponentLifecycle};
use crate::assets::prefabs;
use crate::turbe::position::Position;

use super::entity::Entity;

#[derive(Debug, Copy, Clone, PartialEq, BorshSerialize, BorshDeserialize)]
pub struct SceneData {
    pub active_scene : Scenes,
    pub is_loaded : bool
}

#[derive(Debug, Copy, Clone, PartialEq, BorshSerialize, BorshDeserialize)]
pub enum Scenes {
    Title
}

pub fn make_scene (some_scene : Scenes) ->  VecDeque<Entity<Component>>{

    match some_scene {
        Scenes::Title => {make_title_scene()},
        default => {
            return VecDeque::new();
        }
    }

}

pub fn make_title_scene () -> VecDeque<Entity<Component>> {

    let mut ent_vec = VecDeque::new();

    ent_vec.push_front(prefabs::new_text());
    ent_vec.push_front(prefabs::title());
    ent_vec.push_front(prefabs::startButton());
    ent_vec.push_front(prefabs::optionsButton());
    ent_vec.push_front(prefabs::creditsButton());
    ent_vec.push_front(prefabs::exitButton());

    for i in 1..(20 + rand()%20) {

        let some_pos = Position{ 
            x: (1 + (rand() % 255)) as i32 , y: (1 + (rand() % 255)) as i32, rotation : 0
        };
        let some_val : u32 = 5 + rand()%40;

        ent_vec.push_front(prefabs::bee(some_pos, some_val));

    }

    let mut ent = prefabs::comb();

    for i in 0..10 {
        for j in 0..10 {

            ent = prefabs::comb();

            let mut some_point = Position::new();

            some_point.set_x((i* 30) - 10);
            some_point.set_y(-20 + (j * 30));
            
            ent_vec.push_front(prefabs::rot_comb(-2.0, some_point));

        }
    }

    return ent_vec;

}