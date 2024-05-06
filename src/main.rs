use std::collections::BTreeSet;
use std::ops::Sub;

use bevy::log::tracing_subscriber::fmt::time;
use bevy::prelude::*;
use bevy::tasks::futures_lite::stream::iter;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(HelloPlugin)
        .add_systems(Update, update_kat)
        .run();
}

#[derive(Component)]
struct Person;

#[derive(Component, Debug, PartialEq)]
struct Name(&'static str);

#[derive(Component, Debug, PartialEq, Clone)]
enum Position {
    Imaginary(&'static str),
    Position2D(Position2D),
    Dead,
}

#[derive(Component, Debug, PartialEq, Clone)]
struct Position2D {
    x: f32,
    y: f32,
}

fn hello_world() {
    println!("Hello, World!");
}

fn add_people(mut commands: Commands) {
    commands.spawn((Person, Name("kat"), Position::Imaginary("PC")));
    commands.spawn((
        Person,
        Name("Kathryn Sakura"),
        Position::Position2D(Position2D { x: 0.0, y: 0.0 }),
    ));
    commands.spawn((
        Person,
        Name("Ms Kae Sakura"),
        Position::Position2D(Position2D { x: 0.0, y: 1.0 }),
    ));
    commands.spawn((
        Person,
        Position::Position2D(Position2D { x: 100.0, y: 1.0 }),
    ));
    commands.spawn((Person, Name("Gabriel"), Position::Dead));
    commands.spawn((
        Person,
        Name("Kathleen Sakura"),
        Position::Position2D(Position2D { x: -1.0, y: 1.0 }),
    ));
}

fn get_greeting(name: Option<&Name>, pos: &Position) -> String {
    match (name, pos) {
        (Some(name), Position::Imaginary(place)) => format!("Hello, {name:?} in {place:?}"),
        (Some(name), Position::Position2D(xy)) => format!("Hello, {name:?} at {xy:?}"),
        (Some(name), Position::Dead) => format!("Goodbye, {name:?}"),
        (None, Position::Imaginary(place)) => format!("theres something in {place:?}"),
        (None, Position::Position2D(xy)) => format!("theres something at {xy:?}"),
        (None, Position::Dead) => unreachable!(),
    }
}

#[derive(Resource)]
struct GreetTimer(Timer);

#[derive(Resource)]
struct GreetPreviousGreetings(BTreeSet<String>);

impl GreetPreviousGreetings {
    #[must_use]
    fn update(&mut self, new: BTreeSet<String>) -> BTreeSet<String> {
        let delta = self.0.sub(&new);
        self.0 = new;

        delta
    }
}

fn greet(
    time: Res<Time>,
    mut timer: ResMut<GreetTimer>,
    mut previous_greetings: ResMut<GreetPreviousGreetings>,
    query: Query<(Option<&Name>, &Position), With<Person>>,
) {
    if !timer.0.tick(time.delta()).just_finished() {
        return;
    }

    let greetings = query
        .iter()
        .map(|(name, position)| get_greeting(name, position))
        .collect();

    for new_greeting in previous_greetings.update(greetings) {
        info!("{new_greeting}");
    }
}

fn greet_all(query: Query<(Option<&Name>, &Position), With<Person>>) {
    query
        .iter()
        .map(|(name, position)| get_greeting(name, position))
        .for_each(|greeting| info!("{greeting}"));
}

fn update_kat(mut query: Query<(&Name, &mut Position), With<Person>>) {
    for (_, mut position) in query.iter_mut().filter(|(name, _)| name == &&Name("kat")) {
        let pos = position.clone();
        match pos {
            Position::Imaginary(_) => {
                *position = Position::Position2D(Position2D { x: 100.0, y: 12.0 });
            }
            Position::Position2D(_) => *position = Position::Imaginary("pc"),
            Position::Dead => {}
        }
    }
}

pub struct HelloPlugin;

impl Plugin for HelloPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(GreetTimer(Timer::from_seconds(2.0, TimerMode::Repeating)))
            .insert_resource(GreetPreviousGreetings(BTreeSet::new()))
            .add_systems(Startup, (hello_world, add_people, greet_all).chain())
            .add_systems(Update, greet);
    }
}
