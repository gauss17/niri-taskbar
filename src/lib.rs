use std::collections::{BTreeMap, BTreeSet};

use button::Button;
use config::Config;
use error::Error;
use state::State;
use waybar_cffi::{
    Module,
    gtk::{
        self, Orientation,
        glib::MainContext,
        traits::{BoxExt, ContainerExt, StyleContextExt, WidgetExt},
    },
    waybar_module,
};

mod button;
mod config;
mod error;
mod icon;
mod niri;
mod state;

struct TaskbarModule {}

impl Module for TaskbarModule {
    type Config = Config;

    fn init(info: &waybar_cffi::InitInfo, config: Config) -> Self {
        let module = Self {};
        let state = State::new(config);

        if let Err(e) = init(info, state) {
            eprintln!("niri taskbar module init error: {e:?}");
        }

        module
    }
}

waybar_module!(TaskbarModule);

fn init(info: &waybar_cffi::InitInfo, state: State) -> Result<(), Error> {
    // Set up the box that we'll use to contain the actual window buttons.
    let root = info.get_root_widget();
    let container = gtk::Box::new(Orientation::Horizontal, 0);
    container.style_context().add_class("niri-taskbar");
    root.add(&container);

    // We need to spawn a task to receive the window snapshots and update the container.
    let context = MainContext::default();
    let stream = state.niri().window_stream()?;

    context.spawn_local(async move {
        // It's inefficient to recreate every button every time, so we keep them in a cache and
        // just reorder the container as we update based on each snapshot. This also avoids
        // flickering while rendering, since the images don't have to be reloaded from disk.
        let mut buttons = BTreeMap::new();

        while let Some(windows) = stream.next().await {
            // We need to track which, if any, windows are no longer present.
            let mut omitted = buttons.keys().copied().collect::<BTreeSet<_>>();

            for window in windows.into_iter() {
                let button = buttons.entry(window.id).or_insert_with(|| {
                    let button = Button::new(&state, &window);

                    // Implicitly adding the button widget to the box as we create it simplifies
                    // reordering, since it means we can just do it as we go.
                    container.add(button.widget());
                    button
                });

                // Update the window properties.
                button.set_focus(window.is_focused);
                button.set_title(window.title.as_deref());

                // Ensure we don't remove this button from the container.
                omitted.remove(&window.id);

                // Since we get the windows in order in the snapshot, we can just push this to the
                // back and then let other widgets push in front as we iterate.
                container.reorder_child(button.widget(), -1);
            }

            // Remove any windows that no longer exist.
            for id in omitted.into_iter() {
                if let Some(button) = buttons.remove(&id) {
                    container.remove(button.widget());
                }
            }

            // Ensure everything is rendered.
            container.show_all();
        }
    });

    Ok(())
}
