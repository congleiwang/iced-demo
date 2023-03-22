use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Post {
    pub id: i32,
    pub user_id: i32,
    pub title: String,
    pub body: String,
}

impl Post {
    pub async fn fetch_all() -> Result<Vec<Post>, String> {
        let url = String::from("https://jsonplaceholder.typicode.com/posts/");
        reqwest::get(&url).await.map_err(|e| e.to_string())?
            .json().await.map_err(|e| e.to_string())
    }

    pub async fn fetch(id: i32) -> Result<Post, String> {
        let url = format!("https://jsonplaceholder.typicode.com/posts/{}", id);
        reqwest::get(&url).await.map_err(|_| String::new())?
            .json().await.map_err(|_| String::new())
    }
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Comment {
    pub post_id: i32,
    pub id: i32,
    pub name: String,
    pub email: String,
    pub body: String,
}

impl Comment {
    pub async fn fetch_for_post(id: i32) -> Result<Vec<Comment>, String> {
        let url = format!(
            "https://jsonplaceholder.typicode.com/posts/{}/comments/",
            id
        );
        reqwest::get(&url)
            .await
            .map_err(|_| String::new())?
            .json()
            .await
            .map_err(|_| String::new())
    }
}

#[cfg(test)]
mod test {
    use crate::data::{Comment, Post};

    #[test]
    fn fetch_all() {
        let r = tokio_test::block_on(Post::fetch_all());
        match r {
            Ok(ok) => {
                ok.iter().for_each(|post| {
                    println!("post: {:?}", post);
                });
            }
            Err(e) => {
                println!("error: {}", e)
            }
        }
    }

    #[test]
    fn fetch() {
        let r = tokio_test::block_on(Post::fetch(1));
        match r {
            Ok(post) => {
                println!("post: {:?}", post);
            }
            Err(e) => {
                println!("error: {}", e)
            }
        }
    }

    #[test]
    fn fetch_for_post() {
        let r = tokio_test::block_on(Comment::fetch_for_post(1));
        match r {
            Ok(cs) => {
                cs.iter().for_each(|c| {
                    println!("ci: {:?}", c);
                });
            }
            Err(e) => {
                println!("error: {}", e)
            }
        }
    }
}