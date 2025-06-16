use std::char::CharTryFromError;

use crate::turbe::{self, components::comp_orbit::OrbitComponent, position::Position};
use turbe::{entity::Entity, component::{Component, ComponentLifecycle}, border::Border};

use turbe::components::{comp_rect::RectangleComponent, comp_spr::SpriteComponent, comp_text::TextComponent, comp_move::MoveComponent};

pub fn new_text () -> Entity<Component> {

    let mut ent = Entity::new("some text".to_string(), vec![]);

    ent.add_component(TextComponent::new("Buzzing Man".to_string()));

    return ent;

}

pub fn new_rect () -> Entity<Component> {
    
    let mut ent = Entity::new("some rect".to_string(), vec![]);

    ent.add_component(RectangleComponent::new_rect(10, 25, 0x123456ff));
    ent.add_component(MoveComponent::new(-1));

    ent.set_layer(3);

    return ent;

}

pub fn new_spr () -> Entity<Component> {

    let mut ent = Entity::new("some image".to_string(), vec![]);

    ent.transform.set_x(100);

    ent.add_component(SpriteComponent::new_component("smile".to_string()));

    return ent;

}

pub fn comb () -> Entity<Component> {

    let mut ent = Entity::new("comb".to_string(), vec![]);

    ent.add_component(SpriteComponent::new_component("comb".to_string()));
    ent.add_component(MoveComponent::new(2));

    return ent;

}

pub fn title() -> Entity<Component> {
    let mut ent = Entity::new("title".to_string(), vec![]);
    ent.transform.nudge_x(10);
    ent.transform.nudge_y(5);
    ent.set_layer(10);

    let mut rect = RectangleComponent::new_base();
    rect.transform.set_width(130);
    rect.transform.set_height(80);
    rect.color = 0xffffffc0;
    rect.transform.nudge_x(-2);
    rect.transform.nudge_y(-2);

    rect.border.set_radius(10);

    ent.add_component(Component::Rectangle(rect));

    let mut spr = SpriteComponent::new("title".to_string());
    spr.transform.size.set_scale_x(0.4);
    spr.transform.size.set_scale_y(0.4);

    ent.add_component(Component::Sprite(spr));

    return ent;
}

pub fn startButton() -> Entity<Component> {

    let mut ent = Entity::new("start".to_string(), vec![]);
    ent.set_layer(10);

    ent.transform.set_x(10);
    ent.transform.set_y(100);
    ent.transform.set_scale_x(0.5);
    ent.transform.set_scale_y(0.5);

    let mut rect = RectangleComponent::new_base();

    rect.transform.set_width(120);
    rect.transform.set_height(60);
    rect.border.set_radius(8);

    rect.color = 0xffffffc0;
    rect.transform.nudge_x(-2);
    rect.transform.nudge_y(-2);

    ent.add_component(Component::Rectangle(rect));

    let mut spr = SpriteComponent::new("start".to_string());
    spr.transform.nudge_x(3);
    spr.transform.nudge_y(3);

    ent.add_component(Component::Sprite(spr));

    return ent;

}

pub fn optionsButton() -> Entity<Component> {

    let mut ent = Entity::new("options".to_string(), vec![]);
    ent.set_layer(10);

    ent.transform.set_x(10);
    ent.transform.set_y(135);
    ent.transform.set_scale_x(0.5);
    ent.transform.set_scale_y(0.5);

    let mut rect = RectangleComponent::new_base();

    rect.transform.set_width(120);
    rect.transform.set_height(40);
    rect.border.set_radius(8);

    rect.color = 0xffffffc0;
    rect.transform.nudge_x(-2);
    rect.transform.nudge_y(-2);

    ent.add_component(Component::Rectangle(rect));

    let mut spr = SpriteComponent::new("options".to_string());
    spr.transform.nudge_x(3);
    spr.transform.nudge_y(-3);

    ent.add_component(Component::Sprite(spr));

    return ent;

}

pub fn creditsButton() -> Entity<Component> {

    let mut ent = Entity::new("credits".to_string(), vec![]);
    ent.set_layer(10);

    ent.transform.set_x(10);
    ent.transform.set_y(160);
    ent.transform.set_scale_x(0.5);
    ent.transform.set_scale_y(0.5);

    let mut rect = RectangleComponent::new_base();

    rect.transform.set_width(120);
    rect.transform.set_height(50);
    rect.border.set_radius(8);

    rect.color = 0xffffffc0;
    rect.transform.nudge_x(-2);
    rect.transform.nudge_y(-2);

    ent.add_component(Component::Rectangle(rect));

    let mut spr = SpriteComponent::new("credits".to_string());
    spr.transform.nudge_x(3);
    spr.transform.nudge_y(-3);

    ent.add_component(Component::Sprite(spr));

    return ent;

}

pub fn exitButton() -> Entity<Component> {

    let mut ent = Entity::new("exit".to_string(), vec![]);
    ent.set_layer(10);

    ent.transform.set_x(10);
    ent.transform.set_y(190);
    ent.transform.set_scale_x(0.5);
    ent.transform.set_scale_y(0.5);

    let mut rect = RectangleComponent::new_base();

    rect.transform.set_width(120);
    rect.transform.set_height(60);
    rect.border.set_radius(8);

    rect.color = 0xffffffc0;
    rect.transform.nudge_x(-2);
    rect.transform.nudge_y(-2);

    ent.add_component(Component::Rectangle(rect));

    let mut spr = SpriteComponent::new("quit".to_string());
    spr.transform.nudge_x(3);
    spr.transform.nudge_y(3);

    ent.add_component(Component::Sprite(spr));

    return ent;

}

pub fn bee(some_pos : Position, some_val : u32) -> Entity<Component> {

    let mut ent = Entity::new("Bee".to_string(), vec![]);
    ent.set_layer(1);

    let mut spr = SpriteComponent::new("bee".to_string());

    ent.add_component(Component::Sprite(spr));

    let mut orb = OrbitComponent::new(some_val as f32);
    orb.orbit_position = some_pos;
    ent.add_component(Component::Orbit(orb));

    return ent;

}