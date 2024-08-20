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
                InGameSet::MutateBoard,
                // Want to flush in between these systemsets? look at the comment below
                InGameSet::CheckWincondition,
                InGameSet::LogState,
            )
                .chain(),
        );
        // app.add_systems(update,
        //     apply_deferred
        //         .before(InGameSet::MutateBoard)
        //         .after(InGameSet::CheckWincondition)
        //     );
    }
}
