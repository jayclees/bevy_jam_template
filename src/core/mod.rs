//! Foundational features and cross-cutting concerns.

pub mod asset;
pub mod audio;
pub mod camera;
#[cfg(feature = "dev")]
pub mod dev;
pub mod pause;
pub mod physics;
pub mod state;
pub mod window;

use bevy::log::LogPlugin;

use crate::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.configure::<UpdateSystems>();

    // Add Bevy plugins.
    app.add_plugins(
        DefaultPlugins
            .build()
            // TODO: Doing this instead of `.replace` because `window::plugin` requires `AssetPlugin` to load its config.
            .disable::<AssetPlugin>()
            .add_after::<LogPlugin>(asset::plugin)
            .add_after::<LogPlugin>(state::plugin)
            .replace::<WindowPlugin>(window::plugin)
            .set(ImagePlugin::default_nearest()),
    );

    // Add other core plugins.
    app.add_plugins((
        audio::plugin,
        camera::plugin,
        #[cfg(feature = "dev")]
        dev::plugin,
        pause::plugin,
        physics::plugin,
    ));
}

/// Game logic steps for the [`Update`] schedule.
#[derive(SystemSet, Clone, Eq, PartialEq, Hash, Debug)]
pub enum UpdateSystems {
    /// Synchronize start-of-frame values.
    SyncEarly,
    /// Tick timers.
    TickTimers,
    /// Record player and AI input.
    RecordInput,
    /// Step game logic.
    Update,
    /// Handle events emitted this frame.
    HandleEvents,
    /// Apply late commands.
    ApplyCommands,
    /// Synchronize end-of-frame values.
    SyncLate,
}

impl Configure for UpdateSystems {
    fn configure(app: &mut App) {
        app.configure_sets(
            Update,
            (
                Self::SyncEarly,
                Self::TickTimers,
                Self::Update,
                Self::RecordInput,
                Self::HandleEvents,
                Self::ApplyCommands,
                Self::SyncLate,
            )
                .chain(),
        );
    }
}
