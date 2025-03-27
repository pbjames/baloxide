use bevy::prelude::*;

#[derive(Component)]
struct Person;

#[derive(Component)]
struct Name(String);

pub struct HelloPlugin;

impl Plugin for HelloPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, add_people)
            .add_systems(Update, (hello_world, (update_people, greet_people).chain()));
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

fn update_people(mut query: Query<&mut Name, With<Person>>) {
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
