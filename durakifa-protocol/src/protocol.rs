mod components;
mod messages;

pub use self::{
    components::{name::Name, owner::Owner, player::Player, room::Room, user::User},
    messages::lobby::{
        authorize::Authorize, create_room::CreateRoom, join_room::JoinRoom, leave_room::LeaveRoom,
        own_user::OwnUser, register_user::RegisterUser,
    },
};
use naia_shared::Protocolize;

#[derive(Protocolize)]
pub enum Protocol {
    Authorize(Authorize),
    CreateRoom(CreateRoom),
    JoinRoom(JoinRoom),
    LeaveRoom(LeaveRoom),
    Name(Name),
    OwnUser(OwnUser),
    Owner(Owner),
    Player(Player),
    RegisterUser(RegisterUser),
    Room(Room),
    User(User),
}
