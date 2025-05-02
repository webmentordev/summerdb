use actix_nuxt_sql_api::generate_uid;
use actix_web::{App, HttpResponse, HttpServer, Responder, Result, get, web};
use rusqlite::Connection;
use serde::{Deserialize, Serialize};
use std::path::Path;

#[derive(Deserialize, Serialize)]
struct User {
    name: String,
    age: String,
    street: String,
    postal_code: String,
}

#[derive(Serialize)]
struct Version {
    actix: String,
    serde: String,
    rusqlite: String,
    serde_json: String,
}

#[derive(Serialize)]
struct ApiResponse {
    message: String,
    status: u8,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Server is running at: http://127.0.0.1:8080");
    HttpServer::new(|| {
        App::new().service(index).service(
            web::scope("/api")
                .route("/setup", web::get().to(setup))
                .route("/user", web::get().to(user))
                .route("/users", web::get().to(get_users))
                .route("/create/user", web::post().to(create_user)),
        )
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}

async fn setup() -> Result<impl Responder> {
    let file_exists = Path::new("users.db").exists();
    if !file_exists {
        let conn = Connection::open("users.db").unwrap();
        conn.execute(
            "CREATE TABLE IF NOT EXISTS users (
                id INTEGER PRIMARY KEY,
                u_id VARCHAR(255) NOT NULL UNIQUE,
                name VARCHAR(255) NOT NULL,
                age VARCHAR(255) NOT NULL,
                street VARCHAR(255) NOT NULL,
                postal_code VARCHAR(255) NOT NULL
            )",
            [],
        )
        .unwrap();

        let result = ApiResponse {
            message: "Setup completed. Database has been created!".to_string(),
            status: 200,
        };
        Ok(HttpResponse::Ok().json(result))
    } else {
        let result = ApiResponse {
            message: "Database already exist!".to_string(),
            status: 200,
        };
        Ok(HttpResponse::Ok().json(result))
    }
}

#[get("/")]
async fn index() -> Result<impl Responder> {
    let result = Version {
        actix: "4".to_string(),
        serde: "0.35.0".to_string(),
        rusqlite: "1.0".to_string(),
        serde_json: "1.0".to_string(),
    };
    Ok(web::Json(result))
}

async fn user() -> Result<impl Responder> {
    let result = User {
        name: String::from("Ahmer"),
        age: 25.to_string(),
        street: String::from("Aprt#02, Food Street 3"),
        postal_code: String::from("4E3RT"),
    };
    Ok(web::Json(result))
}

async fn create_user(user: web::Json<User>) -> Result<impl Responder> {
    let conn = Connection::open("users.db").unwrap();
    let u_id = generate_uid();
    conn.execute(
        "INSERT INTO users (u_id, name, age, street, postal_code) 
         VALUES (?1, ?2, ?3, ?4, ?5)",
        &[
            &u_id,
            &user.name,
            &user.age,
            &user.street,
            &user.postal_code,
        ],
    )
    .unwrap();

    let result = ApiResponse {
        message: "User created successfully".to_string(),
        status: 201,
    };
    Ok(HttpResponse::Ok().json(result))
}

async fn get_users() -> Result<impl Responder> {
    let conn = Connection::open("users.db").unwrap();

    let mut stmt = conn
        .prepare("SELECT id, u_id, name, age, street, postal_code FROM users ORDER BY id DESC")
        .unwrap();
    let user_rows = stmt
        .query_map([], |row| {
            Ok(User {
                name: row.get(2)?,
                age: row.get(3)?,
                street: row.get(4)?,
                postal_code: row.get(5)?,
            })
        })
        .unwrap();

    let mut users = Vec::new();
    for user in user_rows {
        users.push(user.unwrap());
    }

    let result = Some(users);

    Ok(HttpResponse::Ok().json(result))
}
