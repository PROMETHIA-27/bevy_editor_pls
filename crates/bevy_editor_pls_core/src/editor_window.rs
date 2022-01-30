use bevy::prelude::{App, World};
use bevy::utils::HashMap;
use bevy_inspector_egui::egui;
use std::any::{Any, TypeId};

use crate::editor::EditorWindowState;

pub trait EditorWindow: 'static {
    type State: Default + Any + Send + Sync;

    const NAME: &'static str;
    const DEFAULT_SIZE: (f32, f32) = (0.0, 0.0);

    fn ui(world: &mut World, cx: EditorWindowContext, ui: &mut egui::Ui);
    fn menu_ui(world: &mut World, mut cx: EditorWindowContext, ui: &mut egui::Ui) {
        let _ = world;

        if ui.button(Self::NAME).clicked() {
            cx.open_floating_window::<Self>();
            ui.close_menu();
        }
    }

    fn app_setup(app: &mut App) {
        let _ = app;
    }
}

pub struct EditorWindowContext<'a> {
    pub(crate) window_states: &'a mut HashMap<TypeId, EditorWindowState>,
    pub(crate) internal_state: &'a mut crate::editor::EditorInternalState,
}
impl EditorWindowContext<'_> {
    pub fn state_mut<W: EditorWindow>(&mut self) -> Option<&mut W::State> {
        self.window_states
            .get_mut(&TypeId::of::<W>())
            .and_then(|s| s.downcast_mut::<W::State>())
    }
    pub fn state<W: EditorWindow>(&self) -> Option<&W::State> {
        self.window_states
            .get(&TypeId::of::<W>())
            .and_then(|s| s.downcast_ref::<W::State>())
    }

    pub fn open_floating_window<W: ?Sized + EditorWindow>(&mut self) {
        let floating_window_id = self.internal_state.next_floating_window_id();
        let window_id = std::any::TypeId::of::<W>();
        self.internal_state
            .floating_windows
            .push(crate::editor::FloatingWindow {
                window: window_id,
                id: floating_window_id,
                original_panel: None,
                initial_position: None,
            });
    }
}
