use crate::{app::AppPlugin, cli::Args, project::LoadProjectData};
use bevy::ecs::system::SystemState;
use bevy::prelude::*;
use bevy::winit::{EventLoopProxy, EventLoopProxyWrapper};
use js_sys::Function;
use wasm_bindgen::prelude::*;

// wasm-pack build --release --target web --out-dir web/pkg

pub(super) fn plugin(app: &mut App) {
    app.add_systems(Startup, startup);
}

#[wasm_bindgen]
pub struct Terp {
    app: App,
}

#[wasm_bindgen]
struct TerpCallback {
    resolve: Function,
}

#[wasm_bindgen]
struct TerpLoader {
    event_loop: EventLoopProxy<LoadProjectData>,
}

#[wasm_bindgen]
impl TerpLoader {
    pub fn load(&self, project: Vec<u8>) -> Result<(), String> {
        if self.event_loop.send_event(LoadProjectData(project)).is_ok() {
            Ok(())
        } else {
            Err("Event loop closed".into())
        }
    }
}

#[wasm_bindgen]
impl Terp {
    pub fn run(&mut self) {
        self.app.run();
    }
}

fn create(plugin: AppPlugin, resolve: Function) -> Terp {
    let mut app = plugin.app();
    app.insert_non_send_resource(TerpCallback { resolve });
    Terp { app }
}

#[wasm_bindgen]
pub fn create_editor(resolve: Function) -> Terp {
    create(AppPlugin::Editor(Args::default()), resolve)
}

#[wasm_bindgen]
pub fn create_player(resolve: Function) -> Terp {
    create(AppPlugin::Player(Args::default()), resolve)
}

fn startup(
    world: &mut World,
    params: &mut SystemState<Res<EventLoopProxyWrapper<LoadProjectData>>>,
) {
    let loader = TerpLoader {
        event_loop: params.get(world).clone(),
    };
    if let Some(ref callback) = world.remove_non_send_resource::<TerpCallback>() {
        let _ = callback
            .resolve
            .call1(&JsValue::null(), &JsValue::from(loader));
    }
}
