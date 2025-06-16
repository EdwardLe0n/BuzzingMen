use turbo::prelude::*;
use std::collections::VecDeque;

use crate::turbe::component::{Component, ComponentLifecycle};
use crate::assets::prefabs;

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

    let mut ent = prefabs::comb();

    for i in 0..10 {
        for j in 0..10 {

            ent = prefabs::comb();

            ent.transform.nudge_y((i* 30) - 10);
            ent.transform.nudge_x(-20 + (j * 30));
            
            ent_vec.push_front(ent);

        }
    }

    return ent_vec;

}