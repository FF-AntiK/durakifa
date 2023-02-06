mod components;
mod messages;

pub use self::{
    components::{name::Name, owner::Owner, player::Player, room::Room, user::User},
    messages::{add::Add, auth::Auth, join::Join, leave::Leave, own::Own, register::Register},
};
use naia_shared::Protocolize;

#[derive(Protocolize)]
pub enum Protocol {
    Add(Add),
    Auth(Auth),
    Join(Join),
    Leave(Leave),
    Name(Name),
    Own(Own),
    Owner(Owner),
    Player(Player),
    Register(Register),
    Room(Room),
    User(User),
}
