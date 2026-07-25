use bevy::app::App;
use bevy::app::Plugin;

pub trait AppExt {
    fn require_plugin<T: Plugin>(&mut self) -> &mut Self;
}

impl AppExt for App {
    #[track_caller]
    fn require_plugin<T: Plugin>(&mut self) -> &mut Self {
        if !self.is_plugin_added::<T>() {
            panic!(
                "{} must be added before this plugin",
                std::any::type_name::<T>()
            );
        }
        self
    }
}
