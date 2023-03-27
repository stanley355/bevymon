use bevy::prelude::*;

#[derive(Debug)]
pub enum FramePosition {
    Left,
    Top,
    Bottom,
    Right,
}

#[derive(Debug)]
pub struct Frame {
    name: String,
    position: FramePosition,
}

impl Frame {
    fn frame_size(&self, window: &Window) -> Size {
        match self.position {
            FramePosition::Left | FramePosition::Right => {
                Size::new(Val::Px(0.1 * window.width()), Val::Percent(100.))
            }
            FramePosition::Top | FramePosition::Bottom => {
                Size::new(Val::Px(0.8 * window.width()), Val::Percent(10.))
            }
        }
    }

    fn frame_bundle(&self, window: &Window) -> NodeBundle {
        match self.position {
            FramePosition::Left => NodeBundle {
                style: Style {
                    size: Self::frame_size(&self, window),
                    ..Default::default()
                },
                background_color: Color::WHITE.into(),
                ..Default::default()
            },
            FramePosition::Right => NodeBundle {
                style: Style {
                    size: Self::frame_size(&self, window),
                    position: UiRect::left(Val::Px(-0.8 * window.width())),
                    ..Default::default()
                },
                background_color: Color::WHITE.into(),
                ..Default::default()
            },
            FramePosition::Top => NodeBundle {
                style: Style {
                    size: Self::frame_size(&self, window),
                    ..Default::default()
                },
                background_color: Color::BLACK.into(),
                ..Default::default()
            },
            FramePosition::Bottom => NodeBundle {
                style: Style {
                    size: Self::frame_size(&self, window),
                    position: UiRect {
                        top: Val::Px(0.9 * window.height()),
                        left: Val::Px(-0.8 * window.width()),
                        ..Default::default()
                    },
                    ..Default::default()
                },
                background_color: Color::BLACK.into(),
                ..Default::default()
            },
        }
    }

    fn new(name: String, position: FramePosition, window: &Window) -> (Name, NodeBundle) {
        let frame = Frame {
            name: name.clone(),
            position,
        };

        let frame_name = Name::new(name);

        (frame_name, frame.frame_bundle(window))
    }

    pub fn frame_setup(mut commands: Commands, query: Query<&Window>) {
        let window = query.single();

        let left_frame = Self::new("left_frame".to_string(), FramePosition::Left, window);
        let top_frame = Self::new("top_frame".to_string(), FramePosition::Top, window);
        let bot_frame = Self::new("bottom_frame".to_string(), FramePosition::Bottom, window);
        let right_frame = Self::new("right_frame".to_string(), FramePosition::Right, window);

        commands.spawn((left_frame.0, left_frame.1));
        commands.spawn((top_frame.0, top_frame.1));
        commands.spawn((bot_frame.0, bot_frame.1));
        commands.spawn((right_frame.0, right_frame.1));
    }
}
