use std::borrow::Cow;

use bevy::prelude::*;
use bevy::render::wireframe::WireframeConfig;

use crate::{plugin::EditorState, systems::EditorEvent, EditorSettings, WireframeMode};
use bevy_inspector_egui::{
    bevy_egui::EguiContext,
    egui::{self, menu},
    Context, Inspectable, WorldInspectorParams,
};

pub(crate) fn menu_system(
    egui_context: Res<EguiContext>,
    mut editor_settings: ResMut<EditorSettings>,
    mut editor_events: ResMut<Events<EditorEvent>>,
    mut inspector_params: ResMut<WorldInspectorParams>,
    mut wireframe_config: ResMut<WireframeConfig>,
) {
    egui::TopPanel::top("editor-pls top panel").show(&egui_context.ctx, |ui| {
        menu::bar(ui, |ui| {
            menu::menu(ui, "Inspector", |ui| {
                egui::Grid::new("inspector settings").show(ui, |ui| {
                    checkbox(ui, &mut inspector_params.enabled, "World Inspector");
                    ui.end_row();
                    checkbox(ui, &mut editor_settings.click_to_inspect, "Click to inspect");
                    ui.end_row();

                    // ui.label("Wireframe mode");
                    // let context = bevy_inspector_egui::Context::new_shared(&egui_context.ctx);
                    // editor_settings.wireframe_mode.ui(ui, (), &context);
                    let mut enable_wireframe = match editor_settings.wireframe_mode {
                        WireframeMode::None => false,
                        WireframeMode::WithWireframeComponent => todo!(),
                        WireframeMode::All => true,
                    };
                    checkbox(ui, &mut enable_wireframe, "Wireframes");
                    editor_settings.wireframe_mode = match enable_wireframe {
                        false => WireframeMode::None,
                        true => WireframeMode::All,
                    };
                    wireframe_config.global = enable_wireframe;
                });
            });

            if !editor_settings.events_to_send.is_empty() {
                menu::menu(ui, "Events", |ui| {
                    for (index, (name, _)) in editor_settings.events_to_send.iter().enumerate() {
                        if ui.button(name).clicked() {
                            editor_events.send(EditorEvent::SendEvent(index));
                        }
                    }
                });
            }

            if !editor_settings.state_transition_handlers.is_empty() {
                menu::menu(ui, "States", |ui| {
                    for (index, (name, _)) in editor_settings.state_transition_handlers.iter().enumerate() {
                        if ui.button(name).clicked() {
                            editor_events.send(EditorEvent::StateTransition(index));
                        }
                    }
                });
            }
        });
    });
}

pub(crate) fn currently_inspected_system(world: &mut World, resources: &mut Resources) {
    let egui_context = resources.get::<EguiContext>().unwrap();
    let editor_settings = resources.get_mut::<EditorSettings>().unwrap();
    let mut editor_state = resources.get_mut::<EditorState>().unwrap();

    if !editor_settings.click_to_inspect {
        return;
    }

    let mut currently_inspected = match editor_state.currently_inspected {
        Some(entity) => entity,
        None => return,
    };

    let context = Context::new(&egui_context.ctx, world, resources);
    let mut is_open = true;
    egui::Window::new("Inspector")
        .open(&mut is_open)
        .id(egui::Id::new("editor inspector"))
        .show(&egui_context.ctx, |ui| {
            ui.wrap(|ui| {
                ui.style_mut().visuals.override_text_color = Some(ui.style().visuals.widgets.hovered.text_color());
                ui.heading(entity_name(world, currently_inspected).as_ref());
            });

            ui.style_mut().wrap = Some(false);
            inspector_ui(&mut currently_inspected, ui, &context);
        });

    if !is_open {
        editor_state.currently_inspected = None;
    }
}

fn inspector_ui<T: Inspectable>(val: &mut T, ui: &mut egui::Ui, context: &Context) {
    val.ui(ui, Default::default(), context);
}

fn checkbox(ui: &mut egui::Ui, selected: &mut bool, text: &str) {
    if ui.selectable_label(false, text).clicked() {
        *selected = !*selected;
    }
    ui.wrap(|ui| {
        let style = &mut ui.style_mut().visuals.widgets;
        style.inactive.bg_fill = style.active.bg_fill;
        ui.spacing_mut().icon_spacing = 0.0;
        ui.checkbox(selected, "");
    });
}

fn entity_name(world: &World, entity: Entity) -> Cow<'_, str> {
    match world.get::<Name>(entity) {
        Ok(name) => name.as_str().into(),
        Err(_) => format!("Entity {}", entity.id()).into(),
    }
}
