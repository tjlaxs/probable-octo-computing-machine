use bevy::prelude::*;

fn init_status(mut commands: Commands) {
    println!("init_status")
}

fn show_status(git_status: ResMut<GitStatus>) {
    for x in git_status.0.iter() {
        println!("{} - {}", x.0, x.1)
    }
}

#[derive(Resource)]
struct GitStatus(Vec<GitStatusFile>);

struct GitStatusFile(String, String);

#[derive(Resource)]
struct GitStatusTimer(Timer);

pub struct GitStatusPlugin;

impl GitStatusPlugin {}

fn format_git_status_file(gs_file: &str) -> GitStatusFile {
    let trimmed = gs_file.trim();
    let mut split = trimmed.split(' ');
    let git_type = split.next().unwrap();
    let git_file = split.next().unwrap();
    GitStatusFile(git_type.to_string(), git_file.to_string())
}

impl Plugin for GitStatusPlugin {
    fn build(&self, app: &mut App) {
        let git_status =
            " D LICENSE\n M src/main.rs\n?? README.md\n?? src/awesome.rs\n".to_string();
        let files = git_status
            .lines()
            .map(format_git_status_file)
            .collect::<Vec<GitStatusFile>>();
        app.insert_resource(GitStatus(files));
        app.insert_resource(GitStatusTimer(Timer::from_seconds(
            5.0,
            TimerMode::Repeating,
        )));
        app.add_systems(Startup, (init_status, show_status));
    }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(GitStatusPlugin)
        .run();
}
