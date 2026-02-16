//! Demonstrating clipping with border radius

use bevy::{
    color::palettes::css::{BLUE, RED},
    prelude::*,
};

#[derive(Resource)]
struct Marker(Entity);

fn main() {
    let mut app = App::new();

    app.add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, update);

    app.run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);
    commands
        .spawn(
            (Node {
                width: percent(100),
                height: percent(100),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..Default::default()
            }),
        )
        .with_children(|parent| {
            let id = parent
                .spawn((
                    Node {
                        width: px(100),
                        height: px(100),
                        border: UiRect::all(px(10)),
                        border_radius: BorderRadius::MAX,
                        overflow: Overflow::clip(),
                        align_items: AlignItems::Center,
                        justify_content: JustifyContent::Center,
                        ..Default::default()
                    },
                    BorderColor::all(BLUE),
                ))
                .with_children(|parent| {
                    parent.spawn((
                        Node {
                            width: percent(110),
                            height: percent(110),
                            ..Default::default()
                        },
                        BackgroundColor(RED.into()),
                    ));
                })
                .id();

            parent.commands_mut().insert_resource(Marker(id));
        });
}

fn update(mut nodes: Query<&mut Node>, marker: Res<Marker>, keys: Res<ButtonInput<KeyCode>>) {
    let Ok(mut node) = nodes.get_mut(marker.0) else {
        return;
    };

    if keys.just_pressed(KeyCode::KeyT) {
        node.overflow = match node.overflow {
            Overflow {
                x: OverflowAxis::Visible,
                y: OverflowAxis::Visible,
            } => Overflow::clip_y(),
            Overflow {
                x: OverflowAxis::Visible,
                y: OverflowAxis::Clip,
            } => Overflow::clip_x(),
            Overflow {
                x: OverflowAxis::Clip,
                y: OverflowAxis::Visible,
            } => Overflow::clip(),
            _ => Overflow::visible(),
        };
    }
}
