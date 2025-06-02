use bevy::prelude::*;
use std::process::Command;

#[derive(Resource)]
struct GitStatus(Vec<GitStatusFile>);

struct GitStatusFile(GitStatusFileState, String);

#[derive(Debug, PartialEq, Clone)]
enum GitStatusFileState {
    Deleted,
    Modified,
    NotTracked,
    Added,
    ModifiedInBothStages,
    AddedThenModified,
}

impl std::fmt::Display for GitStatusFileState {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let mtc = match self {
            GitStatusFileState::Deleted => " D",
            GitStatusFileState::Modified => " M",
            GitStatusFileState::Added => " A",
            GitStatusFileState::NotTracked => "??",
            GitStatusFileState::ModifiedInBothStages => "MM",
            GitStatusFileState::AddedThenModified => "AM",
        };
        write!(f, "{}", mtc)
    }
}

#[derive(Resource)]
struct GitStatusTimer(Timer);

pub struct GitStatusPlugin;

impl GitStatusPlugin {}

impl Plugin for GitStatusPlugin {
    fn build(&self, app: &mut App) {
        let stdout = Command::new("git")
            .arg("status")
            .arg("--porcelain")
            .output()
            .unwrap()
            .stdout;
        let git_status = str::from_utf8(&stdout).unwrap();
        println!("{}", git_status);
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

fn mod_color(gsf_state: GitStatusFileState) -> TextColor {
    match gsf_state {
        GitStatusFileState::Deleted => Color::srgba(0.9, 0., 0., 0.5).into(),
        GitStatusFileState::Modified => Color::srgba(0.8, 0.8, 0., 0.5).into(),
        GitStatusFileState::Added => Color::srgba(0.1, 0.2, 0.8, 0.5).into(),
        GitStatusFileState::NotTracked => Color::srgba(0.6, 0.6, 0.6, 0.5).into(),
        GitStatusFileState::ModifiedInBothStages => Color::srgba(0., 0.6, 0.6, 0.5).into(),
        GitStatusFileState::AddedThenModified => Color::srgba(0.4, 0.2, 0.8, 0.5).into(),
    }
}

fn init_status(mut commands: Commands) {}

fn spawn_nested_text_bundle(
    builder: &mut ChildSpawnerCommands,
    text_font: TextFont,
    background_color: Color,
    margin: UiRect,
    text: &str,
) {
    builder
        .spawn((
            Node {
                margin,
                padding: UiRect::axes(Val::Px(5.), Val::Px(1.)),
                ..default()
            },
            BackgroundColor(background_color),
        ))
        .with_children(|builder| {
            builder.spawn((Text::new(text), text_font, TextColor::BLACK));
        });
}

fn show_status(mut commands: Commands, status: ResMut<GitStatus>) {
    commands.spawn(Camera2d);

    let font_size = 16.0;
    let text_font = TextFont {
        font_size,
        ..default()
    };

    commands
        .spawn((
            Node {
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Start,
                justify_content: JustifyContent::Start,
                width: Val::Percent(15.),
                height: Val::Percent(100.),
                ..default()
            },
            BackgroundColor(Color::WHITE),
        ))
        .with_children(|builder| {
            for x in status.0.iter() {
                spawn_nested_text_bundle(
                    builder,
                    text_font.clone(),
                    *mod_color(x.0.clone()),
                    UiRect::top(Val::Px(3.)),
                    &format!("{} {}", x.0, x.1),
                );
            }
        });
}

fn format_git_status_file(gs_file: &str) -> GitStatusFile {
    let (git_status, git_file) = gs_file.split_at(2);
    let gsfs = match git_status {
        " D" => GitStatusFileState::Deleted,
        " M" => GitStatusFileState::Modified,
        "??" => GitStatusFileState::NotTracked,
        " A" => GitStatusFileState::Added,
        "MM" => GitStatusFileState::ModifiedInBothStages,
        "AM" => GitStatusFileState::AddedThenModified,
        _ => panic!("Git status file change type not handled"),
    };
    GitStatusFile(gsfs, git_file.to_string())
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(GitStatusPlugin)
        .run();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_parse_status_file() {
        let result = format_git_status_file("?? src/awesome.rs");
        assert_eq!(result.0.to_string(), "??");
        assert_eq!(result.1, "src/awesome.rs");
    }
}
