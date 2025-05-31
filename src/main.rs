use bevy::prelude::*;

fn init_status(mut commands: Commands, git_status: ResMut<GitStatus>) {
    println!("init_status");
    commands.spawn(Camera2d);

    let text_font = TextFont {
        font_size: 32.0,
        ..default()
    };

    commands
        .spawn((
            Node {
                width: Val::Percent(15.),
                height: Val::Percent(80.),
                top: Val::Percent(10.),
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Start,
                justify_items: JustifyItems::Start,
                padding: UiRect::all(Val::Px(12.)),
                row_gap: Val::Px(12.),
                ..Default::default()
            },
            BackgroundColor(Color::WHITE),
        ))
        .with_children(|builder| {
            for x in git_status.0.iter() {
                builder
                    .spawn((
                        Node {
                            width: Val::Percent(15.),
                            height: Val::Px(40.),
                            align_items: AlignItems::Start,
                            justify_items: JustifyItems::Start,
                            flex_direction: FlexDirection::Row,
                            ..Default::default()
                        },
                        BackgroundColor(Color::linear_rgb(30., 30., 30.)),
                    ))
                    .with_children(|builder| {
                        builder.spawn((
                            Text2d::new(x.1.clone()),
                            text_font.clone(),
                            TextColor::BLACK,
                        ));
                    });
            }
        });
}

fn show_status(mut commands: Commands) {}

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_parse_status_file() {
        let result = format_git_status_file("?? src/awesome.rs");
        assert_eq!(result.0, "??");
        assert_eq!(result.1, "src/awesome.rs");
    }
}
