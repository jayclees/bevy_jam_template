use pyri_state::schedule::ResolveStateSet;

use crate::core::dev::DevConfig;
use crate::prelude::*;
use crate::screen::Screen;
use crate::screen::ScreenTime;

pub(super) fn plugin(app: &mut App) {
    app.init_resource::<StateDebugSettings>();

    // Skip to a custom initial screen.
    app.add_systems(
        StateFlush,
        enter_initial_screen
            .in_set(ResolveStateSet::<Screen>::Compute)
            .run_if(Screen::ANY.will_enable()),
    );

    // Extend loading screen.
    app.add_systems(
        Update,
        (
            Screen::Intro
                .on_update(do_not_skip_loading_screen.track_progress::<BevyState<Screen>>()),
            Screen::Loading.on_update(extend_loading_screen.track_progress::<BevyState<Screen>>()),
        ),
    );
}

pub(super) fn on_load(config: &DevConfig, world: &mut World) {
    world.resource_mut::<StateDebugSettings>().log_flush = config.log_state_flush;
}

fn enter_initial_screen(config: ConfigRef<DevConfig>, mut screen: NextMut<Screen>) {
    let config = r!(config.get());
    screen.enter(rq!(config.initial_screen));
}

fn do_not_skip_loading_screen(config: ConfigRef<DevConfig>) -> Progress {
    let config = r!(config.get());
    (config.extend_loading_screen <= 0.0).into()
}

fn extend_loading_screen(config: ConfigRef<DevConfig>, screen_time: Res<ScreenTime>) -> Progress {
    let config = r!(config.get());
    (screen_time.0.as_secs_f32() >= config.extend_loading_screen).into()
}
