use bevy::prelude::*;

#[derive(Component)]
struct Person;

#[derive(Component)]
struct Name(String);

#[derive(Resource)]
struct GreetTimer(Timer);

pub struct HelloPlugin;

impl Plugin for HelloPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(GreetTimer(Timer::from_seconds(2.0, TimerMode::Repeating)))
            .add_systems(Startup, add_people)
            .add_systems(Update, (hello_world, (greet_people).chain()));
    }
}

fn add_people(mut commands: Commands) {
    commands.spawn((Person, Name("Elaina Proctor".to_string())));
    commands.spawn((Person, Name("Renzo Hume".to_string())));
    commands.spawn((Person, Name("Zayna Nieves".to_string())));
}

fn hello_world() {
    println!("hellow roldd2");
}

fn update_people(
    time: Res<Time>,
    mut timer: ResMut<GreetTimer>,
    mut query: Query<&mut Name, With<Person>>,
) {
    if !timer.0.tick(time.delta()).just_finished() {
        return;
    }
    query.iter_mut().for_each(|mut name| {
        if name.0 == "Elaina Proctor" {
            name.0 = "Elaina Sigma".to_string();
        }
    });
}

fn greet_people(query: Query<&Name, With<Person>>) {
    query.iter().for_each(|name| println!("hello {}", name.0));
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(HelloPlugin)
        .run();
}
