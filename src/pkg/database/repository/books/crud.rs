use chrono::Utc;
use sea_orm::{entity::*,DbErr,DbConn,DatabaseConnection,Set,query::*,ActiveModelTrait};

use crate::pkg::database::models::book;

pub async fn create(
    db: &DbConn,
    uuid: &str,
    title: &str,
    author: &str,
    quantity: i32,
) -> Result<book::Model, DbErr> {
    let new_book = book::ActiveModel {
        uuid: Set(uuid.to_string()),
        title: Set(title.to_string()),
        author: Set(author.to_string()),
        quantity: Set(quantity),
        ..Default::default()  // leave created_at and updated_at unfilled
    };
    
    new_book.insert(db).await
}

pub async fn read_all(db: &DatabaseConnection) -> Result<Vec<book::Model>, DbErr> {
    book::Entity::find().all(db).await
}

pub async fn read_by_uuid(db: &DbConn, uuid: &str) -> Result<Option<book::Model>, DbErr> {
    book::Entity::find() // use Entity from book.rs
        .filter(book::Column::Uuid.eq(uuid)) // filter by UUID
        .one(db) // catch one by condition
        .await // wait for the results asynchronously
}

pub async fn update(
    db: &DbConn,
    uuid: &str,
    title: Option<&str>,
    author: Option<&str>,
    quantity: Option<i32>,
) -> Result<book::Model, DbErr> {
    // use book::Entity::find() by UUID
    let model = book::Entity::find()
        .filter(book::Column::Uuid.eq(uuid))
        .one(db)
        .await?
        .ok_or(DbErr::RecordNotFound("Book not found".to_string()))?; // catch Model, and error if not found

    // convert Model to ActiveModel
    let mut active_model = model.into_active_model();

    // update field 
    if let Some(title) = title {
        active_model.title = Set(title.to_string());
    }
    if let Some(author) = author {
        active_model.author = Set(author.to_string());
    }
    if let Some(quantity) = quantity {
        active_model.quantity = Set(quantity);
    }

    // set updated_at
    active_model.updated_at =  Set(Utc::now().naive_utc());

    // Update entity to database
    active_model.update(db).await
}



pub async fn delete(db: &DbConn, uuid: &str) -> Result<u64, DbErr> {
    // use delete_many() for delete more than on
    book::Entity::delete_many()
        .filter(book::Column::Uuid.eq(uuid)) // filter by UUID
        .exec(db) // execute query delete
        .await // wait by async
        .map(|result| result.rows_affected) // catch sum of line which deleted
}

pub async fn delete_soft(db: &DbConn, uuid: &str) -> Result<book::Model, DbErr> {
    // find book by UUID
    let book = book::Entity::find()
        .filter(book::Column::Uuid.eq(uuid))
        .one(db)
        .await?;

    // check the book, is it found?
    let book = match book {
        Some(book) => book,
        None => return Err(DbErr::RecordNotFound("Book not found".to_string())),
    };

    // change to ActiveModel for update
    let mut active_model: book::ActiveModel = book.into_active_model();
    active_model.deleted_at = Set(Some(Utc::now().naive_utc())); // set deleted_at

    // Update entity to database
    active_model.update(db).await
}
