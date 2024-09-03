use bevy::prelude::*;

#[derive(SystemSet, Hash, Debug, Clone, Eq, PartialEq)]
pub enum InGameSet {
    MutateCamera,
    MutateBoard,
    CheckWincondition,
    LogState,
    HandleMenu,
}

pub struct SchedulePlugin;

impl Plugin for SchedulePlugin {
    fn build(&self, app: &mut App) {
        app.configure_sets(
            Update,
            (
                InGameSet::MutateCamera,
                InGameSet::CheckWincondition,
                // Want to flush in between these systemsets? look at the comment below
                InGameSet::MutateBoard,
                InGameSet::LogState,
            )
                .chain(),
        );
        // app.add_systems(
        //     Update,
        //     apply_deferred
        //         .after(InGameSet::CheckWincondition)
        //         .before(InGameSet::MutateBoard),
        // );
    }
}
