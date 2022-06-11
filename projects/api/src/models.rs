use diesel;
use diesel::prelude::*;
use diesel::pg::PgConnection;

pub mod schema;

use schema::books;
use schema::books::dsl::books as all_books;

//build out model for a data
#[derive(Queryable)]
pub struct Book {
    pub id: i32,
    pub title: String,
    pub author: String,
    pub published: bool,
}

//create new struct for new books
#[derive(Insertable)]
#[table_name = "books"]
pub struct NewBook {
    pub title: String,
    pub author: String,
    pub published: bool,
}

impl Book {
    //show books by id
    pub fn show(id: i32, conn: &PgConnection) -> Vec<Book> {
        all_books
            .find(id)
            .load::<Book>(conn)
            .expect("Error loading book")
    }
    //query method by all
    pub fn all(conn: &PgConnection) -> Vec<Book> {
        all_books
            .order(books::id.desc())
            .load::<Book>(conn)
            .expect ("error loading the books")
    }
    //update pg by id
    pub fn update_by_id(id: i32, conn: &PgConnection, book: NewBook) -> bool {
        use books::dsl::{author as a, published as p, title as t};
        let NewBook {
            title,
            author,
            published,
        } = book;
        //look for specific book by id
        diesel::update(all_books.find(id))
            .set((a.eq(author), p.eq(published), t.eq(title)))
            .get_result::<Book>(conn)
            .is_ok()
    }
    //insert new book
    pub fn insert(book: NewBook, conn: &PgConnection) -> bool {
        diesel::insert_into(books::table)
        .values(&book)
        .execute(conn)
        .is_ok()
    }
    //delet by id
    pub fn delete_by_id(id: i32, conn: &PgConnection) -> bool {
        if Book::show(id,conn).is_empty() {
            return false;
        };
        diesel::delete(all_books.find(id)).execute(conn).is_ok()
    }
    //find book by author
    pub fn all_by_author(author: String, conn: &PgConnection) -> Vec<Book> {
        all_books
            .filter(books::author.eq(author))
            .load::<Book>(conn)
            .expect("Error loading books by author")
    }
}