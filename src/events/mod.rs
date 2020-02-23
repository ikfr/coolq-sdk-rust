mod add_friend_request;
mod add_group_request;
mod discuss_message;
mod friend_add;
mod group_admin;
mod group_ban;
mod group_member_decrease;
mod group_member_increase;
mod group_message;
mod group_upload;
mod private_message;

pub use add_friend_request::*;
pub use add_group_request::*;
pub use discuss_message::*;
pub use friend_add::*;
pub use group_admin::*;
pub use group_ban::*;
pub use group_member_decrease::*;
pub use group_member_increase::*;
pub use group_message::*;
pub use group_upload::*;
pub use private_message::*;

type StartEvent = ();
type ExitEvent = ();
type DisableEvent = ();