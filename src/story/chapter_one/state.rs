use bevy::prelude::States;

#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
pub enum ChapterOneState {
    #[default]
    PreChapter,
    BackgroundScene,
}

#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
pub enum BgSceneState {
    #[default]
    NoImg,
    EggImg,
    MewImg,
    ArceusImg,
    CleanupImg,
}

