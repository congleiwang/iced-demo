use iced::alignment::Vertical;
use iced::Element;
use iced::widget::{Button, Column, Row, Text};
use crate::data::{Comment, Post};
use crate::Message;

pub struct PostCmp {
    pub(crate) post: Post,
}

pub struct CommentCmp {
    pub(crate) comment: Comment,
}

impl CommentCmp {
    pub fn view(&self) -> Element<Message> {
        Column::new()
            .push(Text::new(format!("name:{}", self.comment.name)).size(12))
            .push(Text::new(format!("email:{}", self.comment.email)).size(12))
            .push(Text::new(self.comment.body.to_owned()).size(12))
            .into()
    }
}

impl PostCmp {
    pub fn view(&self) -> Element<Message> {
        Column::new()
            .push(Text::new(format!("id:{}", self.post.id)).size(12))
            .push(Text::new(format!("user_id:{}", self.post.user_id)).size(12))
            .push(Text::new(format!("title:{}", self.post.title)).size(12))
            .push(Text::new(self.post.body.to_owned()).size(12))
            .into()
    }

    pub fn view_in_list(&self) -> Element<Message> {
        let r = Row::new().padding(5).spacing(5);
        r.push(
            Column::new().spacing(5).push(
                Text::new(self.post.title.to_owned())
                    .size(12)
                    .vertical_alignment(Vertical::Center),
            ),
        )
            .push(
                Button::new(Text::new("Detail").size(12))
                    .on_press(Message::GoToDetail(self.post.id))
            )
            .into()
    }
}