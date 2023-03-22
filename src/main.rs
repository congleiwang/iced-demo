use iced::{executor, Application, Command, Element, Renderer, Theme, Settings, Alignment};
use iced::widget::{Button, Column, column, Text};
use crate::data::{Comment, Post};
use crate::view::{CommentCmp, PostCmp};

mod data;
mod view;

fn main() -> iced::Result {
    App::run(Settings::default())
}

struct App {
    route: Route,
    posts: Option<Vec<PostCmp>>,
    post: Option<PostCmp>,
    comments: Option<Vec<CommentCmp>>,
}

impl App {
    fn render_posts(posts: &Vec<PostCmp>) -> Element<Message> {
        // let c = Column::new();
        // let posts: Element<Message> = posts
        //     .iter()
        //     .fold(Column::new().spacing(10), |col, p| col.push(p.view_in_list()),
        //     )
        //     .into();
        // c.push(Text::new(String::from("Posts:")).size(15))
        //     .push(posts).into()
        column(
            posts
                .iter()
                .map(|post| {
                    post.view_in_list()
                })
                .collect(),
        ).into()
    }

    fn render_comments(comments: &Vec<CommentCmp>) -> Element<Message> {
        let c = Column::new();
        let comments: Element<Message> = comments
            .iter()
            .fold(Column::new().spacing(10), |col, c| col.push(c.view()))
            .into();
        c.push(Text::new(String::from("Comments:")).size(15))
            .push(comments)
            .into()
    }
}

impl Application for App {
    type Executor = executor::Default;
    type Message = Message;
    type Theme = Theme;
    type Flags = ();

    fn new(_flags: Self::Flags) -> (Self, Command<Self::Message>) {
        (
            App {
                route: Route::List,
                posts: None,
                post: None,
                comments: None,
            },
            Command::perform(data::Post::fetch_all(), Message::PostsFound),
        )
    }

    fn title(&self) -> String {
        String::from("App - Iced")
    }

    // 收到不同的消息做不同的处理
    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        match message {
            Message::PostsFound(posts) => {
                match posts {
                    Ok(data) => {
                        self.posts = Some(
                            data.into_iter().map(|post| PostCmp { post }).collect(),
                        )
                    }
                    Err(e) => println!("error: {}", e),
                }
                Command::none()
            }
            Message::PostFound(post) => {
                match post {
                    Ok(data) => {
                        self.post = Some(PostCmp {
                            post: data
                        })
                    }
                    Err(_) => (),
                }
                Command::none()
            }
            Message::CommentsFount(comments) => {
                match comments {
                    Ok(data) => {
                        self.comments = Some(data.into_iter().map(|comment| CommentCmp { comment }).collect())
                    }
                    Err(_) => (),
                }
                Command::none()
            }
            Message::GoToList => {
                self.post = None;
                self.comments = None;
                self.route = Route::List;
                Command::perform(Post::fetch_all(), Message::PostsFound)
            }
            Message::GoToDetail(id) => {
                self.route = Route::Detail(id);
                self.posts = None;
                Command::batch(vec![
                    Command::perform(Post::fetch(id), Message::PostFound),
                    Command::perform(Comment::fetch_for_post(id), Message::CommentsFount),
                ])
            }
        }
    }

    fn view(&self) -> Element<'_, Self::Message, Renderer<Self::Theme>> {
        let col = Column::new()
            .max_width(600).spacing(10).padding(10).align_items(Alignment::Center)
            .push(Button::new(Text::new("Home")).on_press(Message::GoToList));
        match self.route {
            Route::List => {
                let posts: Element<_> = match self.posts {
                    None => {
                        Column::new().push(Text::new("loading...".to_owned()).size(15)).into()
                    }
                    Some(ref p) => {
                        App::render_posts(p)
                    }
                };
                col.push(Text::new("Home".to_owned()).size(20))
                    .push(posts)
                    .into()
            }
            Route::Detail(id) => {
                let post: Element<_> = match self.post {
                    None => Column::new()
                        .push(Text::new("loading...".to_owned()).size(15))
                        .into(),
                    Some(ref p) => p.view(),
                };
                let comments: Element<_> = match self.comments {
                    None => Column::new()
                        .push(Text::new("loading...".to_owned()).size(15))
                        .into(),
                    Some(ref c) => App::render_comments(c),
                };

                col.push(Text::new(format!("Post: {}", id)).size(20))
                    .push(post)
                    .push(comments)
                    .into()
            }
        }
    }
}

#[derive(Clone, Debug)]
enum Route {
    List,
    Detail(i32),
}

#[derive(Debug, Clone)]
pub enum Message {
    PostsFound(Result<Vec<Post>, String>),
    PostFound(Result<Post, String>),
    CommentsFount(Result<Vec<Comment>, String>),
    GoToList,
    GoToDetail(i32),
}