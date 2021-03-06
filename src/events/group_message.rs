use std::os::raw::c_char;

use crate::{
    api::{Convert, Flag},
    targets::{
        Anonymous,
        group::Group,
        message::{Message, SendMessage},
        user::User,
    },
};
use crate::api::get_group_member_info_v2;
use crate::targets::group::GroupMember;

#[derive(Debug, Clone)]
pub struct GroupMessageEvent {
    pub sub_type: i32,
    pub anonymous_flag: Flag,
    pub msg: Message,
    pub font: i32,
    pub group: Group,
    pub user: User,
}

impl GroupMessageEvent {
    pub fn new(
        sub_type: i32, msg_id: i32, group_id: i64, user_id: i64, anonymous_flag: *const c_char,
        msg: *const c_char, font: i32,
    ) -> Self {
        GroupMessageEvent {
            sub_type,
            anonymous_flag: Convert::from(anonymous_flag).into(),
            msg: Message::new(msg, msg_id),
            font,
            group: Group::new(group_id),
            user: {
                let mut user = User::new(user_id);
                if let Ok(gm) = get_group_member_info_v2(group_id, user_id, false) {
                    user.set_authority(gm.try_to::<GroupMember>().unwrap().authority);
                }
                user
            },
        }
    }

    pub fn get_message(&self) -> &Message {
        &self.msg
    }

    pub fn is_anonymous(&self) -> bool {
        !self.anonymous_flag.is_empty()
    }

    pub fn get_anonymous(&self) -> std::io::Result<Anonymous> {
        if self.is_anonymous() {
            Anonymous::decode(self.anonymous_flag.as_bytes(), self.group.group_id)
        } else {
            Ok(Anonymous::default())
        }
    }

    pub fn reply(&self, msg: impl ToString) -> crate::api::Result<i32> {
        self.group.send_message(msg)
    }

    pub fn reply_at(&self, msg: impl ToString) -> crate::api::Result<i32> {
        self.group.at(self.user.user_id, msg)
    }
}
