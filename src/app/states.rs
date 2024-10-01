use bevy::prelude::States;

#[derive(Clone, Eq, PartialEq, Debug, Hash, States)]
pub enum MyStates {
    Load(Loading),
    Game,
}

#[derive(Clone, Eq, PartialEq, Debug, Hash)]
pub enum Loading {
    Loading,
    Gen,
}

// #[derive(Clone, Eq, PartialEq, Debug, Hash)]
// pub enum Gaming {
//     Init,
//     Guide,
//     Game,
//     Pause,
//     Result,
// }
