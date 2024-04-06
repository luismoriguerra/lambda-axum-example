use axum::{extract::Path, Json};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize)]
pub struct Book {
    id: Uuid,
    title: String,
}

async fn get_books() -> Json<Vec<Book>> {
    Json(vec![
        Book {
            id: Uuid::new_v4(),
            title: "The Catcher in the Rye".to_string(),
        },
        Book {
            id: Uuid::new_v4(),
            title: "1984".to_string(),
        },
    ])
}

async fn get_book(Path(id): Path<Uuid>) -> Json<Book> {
    Json(Book {
        id,
        title: "The Catcher in the Rye".to_string(),
    })
}

pub(crate) fn router() -> axum::Router {
    axum::Router::new()
        .route("/books", axum::routing::get(get_books))
        .route("/books/:id", axum::routing::get(get_book))
}
