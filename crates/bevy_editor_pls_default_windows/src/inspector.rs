use super::hierarchy::HierarchyWindow;
use bevy::ecs::reflect::ReflectComponent;
use bevy::prelude::{Entity, Mut, World};
use bevy::reflect::{TypeRegistry, DynamicStruct};
use bevy_editor_pls_core::editor_window::{EditorWindow, EditorWindowContext};
use bevy_inspector_egui::egui;
use bevy_inspector_egui::{
    options::EntityAttributes, world_inspector::WorldUIContext, WorldInspectorParams,
};

#[derive(Default)]
pub struct InspectorState {
    component_search: String,
}

struct ComponentTypes(Vec<(String, std::any::TypeId)>);

pub struct InspectorWindow;
impl EditorWindow for InspectorWindow {
    type State = InspectorState;
    const NAME: &'static str = "Inspector";

    fn ui(world: &mut World, mut cx: EditorWindowContext, ui: &mut egui::Ui) {
        let inspected = cx.state::<HierarchyWindow>().unwrap().selected;
        inspector(world, inspected, ui, cx.state_mut::<InspectorWindow>().unwrap());
    }

    fn app_setup(app: &mut bevy::prelude::App) {
        let mut comps = ComponentTypes(vec!());

        let reg = app.world.get_resource::<TypeRegistry>().expect("Bevy reflect must be initialized before editor!");

        for ty in reg.read().iter() {
            if let Some(_) = ty.data::<ReflectComponent>() {
                comps.0.push((ty.short_name().to_string(), ty.type_id()))
            }
        }

        comps.0.sort();

        app.insert_resource(comps);
    }
}

fn inspector(world: &mut World, inspected: Option<Entity>, ui: &mut egui::Ui, state: &mut InspectorState) {
    let inspected = match inspected {
        Some(inspected) => inspected,
        None => {
            ui.label("No entity selected");
            return;
        }
    };

    world.resource_scope(|world, params: Mut<WorldInspectorParams>| {
        let entity_options = EntityAttributes::default();
        WorldUIContext::new(world, None).entity_ui_inner(
            ui,
            inspected,
            &*params,
            egui::Id::new("inspector"),
            &entity_options,
        );
    });

    world.resource_scope(|world, reg: Mut<TypeRegistry>| {
        let reg = reg.read();
        world.resource_scope(|world, comp_types: Mut<ComponentTypes>| {
            ui.vertical_centered_justified(|ui| {
                let width = ui.available_size().x;
                ui.menu_button("Add Component", |ui| {
                    ui.set_max_width(width);
                    ui.text_edit_singleline(&mut state.component_search);
                    egui::ScrollArea::vertical().show_rows(ui, ui.fonts()[egui::TextStyle::Body].row_height(), comp_types.0.len(), |ui, rows| {
                        for (name, id) in comp_types.0[rows].iter() {
                            if name.to_lowercase().contains(&state.component_search.to_lowercase()) {
                                if ui.button(name).clicked() {
                                    let ty = reg.get(*id).unwrap();
    
                                    ty.data::<ReflectComponent>().unwrap().add_component(world, inspected, &DynamicStruct::default());
    
                                    ui.close_menu();
    
                                    state.component_search.clear();
                                }
                            }
                        }
                    });
                });
            });

            ui.label("Don't see your component? Make sure to #[derive(Component, Reflect)], #[reflect(Component)], and register its type with the app!");
        });
    });
}
