mod plugins;

use bevy::{
    asset::{AssetServer, HandleUntyped},
    ecs::world::{Mut, World},
    input::{
        keyboard::KeyboardInput,
        mouse::{MouseButtonInput, MouseMotion},
    },
    prelude::{
        default, info, App, Assets, Camera2dBundle, ClearColor, Color, Commands, CoreStage, Entity,
        EventReader, Handle, Image, ImagePlugin, IntoSystemDescriptor, PluginGroup, ResMut,
        Resource, State, SystemLabel, Vec2,
    },
    sprite::TextureAtlas,
    text::Font,
    window::{
        WindowCloseRequested, WindowDescriptor, WindowMode, WindowPlugin, WindowResizeConstraints,
    },
    DefaultPlugins,
};
use bevy_asset_loader::prelude::{AssetCollection, LoadingState, LoadingStateAppExt};
use durakifa_protocol::protocol::{Authorize, Protocol};
use naia_bevy_client::{
    events::{DespawnEntityEvent, MessageEvent, SpawnEntityEvent},
    shared::{DefaultChannels, SharedConfig},
    Client, ClientConfig, Plugin as ClientPlugin, Stage,
};
use obfstr::obfstr;
use plugins::{
    dimensions::DimensionsPlugin, load::LoadPlugin, lobby::LobbyPlugin, menu::MenuPlugin,
    mouse::MousePlugin, register::RegisterPlugin, room::RoomPlugin, vkeyboard::VKeyboardPlugin,
};
use wasm_bindgen::prelude::wasm_bindgen;

#[cfg(debug_assertions)]
const SRV_ADDR: &str = "127.0.0.1";
#[cfg(not(debug_assertions))]
const SRV_ADDR: &str = env!("SRV_ADDR");

#[cfg(not(debug_assertions))]
const SRV_KEY: &str = env!("SRV_KEY");
#[cfg(debug_assertions)]
const SRV_KEY: &str = "SRV_KEY";

#[cfg(debug_assertions)]
const SRV_PORT: &str = "55500";
#[cfg(not(debug_assertions))]
const SRV_PORT: &str = env!("SRV_PORT");

#[cfg(debug_assertions)]
const SRV_PROT: &str = "http";
#[cfg(not(debug_assertions))]
const SRV_PROT: &str = env!("SRV_PROT");

const WND_CLR: Color = Color::BLACK;
const WND_SZE_MIN_X: f32 = 200.0;
const WND_SZE_MIN_Y: f32 = 220.0;
const WND_SZE_X: f32 = 600.0;
const WND_SZE_Y: f32 = 660.0;
const WND_TTL: &str = "DURAKIFA!";

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
enum AppState {
    Connect,
    Game,
    Load,
    Lobby,
    Register,
    Room,
}

#[derive(AssetCollection, Resource)]
struct FontAssets {
    #[asset(path = "fonts/PressStart2P-vaV7.ttf")]
    regular: Handle<Font>,
}

#[derive(AssetCollection, Resource)]
struct ImageAssets {
    #[asset(path = "images/crown.png")]
    crown: Handle<Image>,
}

#[derive(Clone, Debug, Eq, Hash, PartialEq, Resource, SystemLabel)]
enum InputState {
    Keyboard,
    Mouse,
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
enum NetState {
    Offline,
    Online,
}

#[derive(Default, Resource)]
struct LocalUser {
    entity: Option<Entity>,
}

#[derive(AssetCollection, Resource)]
struct SpriteSheetAssets {
    #[asset(texture_atlas(tile_size_x = 32.0, tile_size_y = 32.0, columns = 10, rows = 17))]
    #[asset(path = "images/keys.png")]
    keys: Handle<TextureAtlas>,
}

fn cleanup(
    mut client: Client<Protocol, DefaultChannels>,
    mut event_reader: EventReader<WindowCloseRequested>,
) {
    for _ in event_reader.iter() {
        client.disconnect();
    }
}

fn connect(mut net_state: ResMut<State<NetState>>) {
    if vec![NetState::Offline].contains(net_state.current()) {
        net_state.set(NetState::Online).unwrap();
    }
}

fn disconnect(mut net_state: ResMut<State<NetState>>) {
    if vec![NetState::Online].contains(net_state.current()) {
        net_state.set(NetState::Offline).unwrap();
    }
}

fn input_keyboard(mut input: EventReader<KeyboardInput>, mut state: ResMut<InputState>) {
    if input.iter().count() > 0 && !vec![InputState::Keyboard].contains(&state) {
        *state = InputState::Keyboard;
    }
}

fn input_mouse(
    mut input_button: EventReader<MouseButtonInput>,
    mut input_motion: EventReader<MouseMotion>,
    mut state: ResMut<InputState>,
) {
    if (input_button.iter().count() > 0 || input_motion.iter().count() > 0)
        && !vec![InputState::Mouse].contains(&state)
    {
        *state = InputState::Mouse;
    }
}

fn setup(mut client: Client<Protocol, DefaultChannels>, mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());

    client.auth(Authorize::new(obfstr!(SRV_KEY).to_string()));
    client.connect(&format!("{}://{}:{}", SRV_PROT, SRV_ADDR, SRV_PORT));
}

#[wasm_bindgen]
pub fn start() {
    App::new()
        .insert_resource(ClearColor(WND_CLR))
        .insert_resource(InputState::Mouse)
        .insert_resource(LocalUser::default())
        .add_loading_state(
            LoadingState::new(AppState::Load)
                .continue_to_state(AppState::Connect)
                .with_collection::<FontAssets>()
                .with_collection::<ImageAssets>()
                .with_collection::<SpriteSheetAssets>(),
        )
        .add_plugins(
            DefaultPlugins
                .set(ImagePlugin::default_nearest())
                .set(WindowPlugin {
                    window: WindowDescriptor {
                        height: WND_SZE_Y,
                        mode: WindowMode::Windowed,
                        resize_constraints: WindowResizeConstraints {
                            min_height: WND_SZE_MIN_Y,
                            min_width: WND_SZE_MIN_X,
                            ..Default::default()
                        },
                        title: WND_TTL.to_string(),
                        width: WND_SZE_X,
                        ..Default::default()
                    },
                    ..default()
                }),
        )
        .add_plugin(ClientPlugin::<Protocol, DefaultChannels>::new(
            ClientConfig::default(),
            SharedConfig::default(),
        ))
        .add_plugin(DimensionsPlugin)
        .add_plugin(LoadPlugin)
        .add_plugin(LobbyPlugin)
        .add_plugin(MenuPlugin)
        .add_plugin(MousePlugin)
        .add_plugin(RegisterPlugin)
        .add_plugin(RoomPlugin)
        .add_plugin(VKeyboardPlugin)
        .add_startup_system(setup)
        .add_state(AppState::Load)
        .add_state(NetState::Offline)
        .add_system(input_keyboard.label(InputState::Keyboard))
        .add_system(input_mouse.label(InputState::Mouse))
        .add_system_to_stage(CoreStage::PostUpdate, cleanup)
        .add_system_to_stage(Stage::Connection, connect)
        .add_system_to_stage(Stage::Disconnection, disconnect)
        .add_system_to_stage(Stage::ReceiveEvents, debug_despawn)
        .add_system_to_stage(Stage::ReceiveEvents, debug_spawn)
        .add_system_to_stage(Stage::ReceiveEvents, update_local_player)
        .run();
}

fn update_local_player(
    client: Client<Protocol, DefaultChannels>,
    mut event_reader: EventReader<MessageEvent<Protocol, DefaultChannels>>,
    mut local_user: ResMut<LocalUser>,
) {
    for event in event_reader.iter() {
        if let MessageEvent(_, Protocol::OwnUser(msg)) = event {
            local_user.entity = msg.user.get(&client);
            info!("local user: {:?}", local_user.entity);
        }
    }
}

fn debug_despawn(mut event_reader: EventReader<DespawnEntityEvent>) {
    for event in event_reader.iter() {
        info!("despawned {:?}", event.0);
    }
}

fn debug_spawn(mut event_reader: EventReader<SpawnEntityEvent>) {
    for event in event_reader.iter() {
        info!("spawned {:?}", event.0);
    }
}
