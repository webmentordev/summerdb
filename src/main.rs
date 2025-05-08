// use actix_nuxt_sql_api::generate_uid;
use actix_web::{App, HttpResponse, HttpServer, Responder, Result, get, web};
// use bcrypt::{DEFAULT_COST, hash, verify};
use bcrypt::{DEFAULT_COST, hash};
use rusqlite::Connection;
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fs;
use std::path::Path;

#[derive(Deserialize, Serialize)]
struct User {
    name: String,
    email: String,
    password: String,
    created_at: Option<String>,
    updated_at: Option<String>,
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
    status: u32,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Create Directory & UsersDB
    create_directory().await.unwrap();
    setup().await.unwrap();

    // Start the Actix Server
    println!("Server is running at: http://127.0.0.1:8080");
    HttpServer::new(|| {
        App::new().service(index).service(
            web::scope("/api")
                .route("/setup", web::get().to(setup))
                .route("/user", web::get().to(user))
                .route("/super-users", web::get().to(get_super_users))
                .route("/create/super-user", web::post().to(create_super_user))
                .route("/create/collection", web::post().to(create_collection)),
        )
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}

// Create databases directory to host sqlite DBs
async fn create_directory() -> Result<()> {
    let db_dir = Path::new("databases");
    if !db_dir.exists() {
        fs::create_dir_all(db_dir)?;
    }
    Ok(())
}

// Create users database
async fn setup() -> Result<impl Responder> {
    let file_exists = Path::new("databases/super_users.db").exists();
    if !file_exists {
        let conn = Connection::open("databases/super_users.db").unwrap();
        conn.execute(
            "CREATE TABLE IF NOT EXISTS super_users (
                id INTEGER PRIMARY KEY,
                name VARCHAR(255) NOT NULL,
                email VARCHAR(255) NOT NULL,
                password VARCHAR(255) NOT NULL,
                created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
                updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
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

// Encrypt Password
fn hash_password(password: &str) -> Result<String, Box<dyn Error>> {
    let hashed = hash(password, DEFAULT_COST)?;
    Ok(hashed)
}
// Decrypt & Verify Encrypted Passwword
// fn verify_password(password: &str, hash: &str) -> Result<bool, Box<dyn Error>> {
//     let result = verify(password, hash)?;
//     Ok(result)
// }

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
    Ok(web::Json(User {
        name: String::from("Name"),
        email: String::from("Email"),
        password: String::from("Password"),
        created_at: Some("04 April 2024, 10:06PM UTC".to_owned()),
        updated_at: Some("04 April 2024, 10:06PM UTC".to_owned()),
    }))
}

async fn create_super_user(user: web::Json<User>) -> Result<impl Responder> {
    let conn = Connection::open("databases/super_users.db").unwrap();
    let hashed_password = hash_password(&user.password)?;
    conn.execute(
        "INSERT INTO super_users (name, email, password) 
         VALUES (?1, ?2, ?3)",
        &[&user.name, &user.email, &hashed_password],
    )
    .unwrap();
    Ok(HttpResponse::Ok().json(ApiResponse {
        message: "User created successfully".to_string(),
        status: 201,
    }))
}

async fn get_super_users() -> Result<impl Responder> {
    let conn = Connection::open("databases/super_users.db").unwrap();

    let mut stmt = conn
        .prepare("SELECT name, email, password, created_at, updated_at FROM super_users ORDER BY id DESC")
        .unwrap();
    let user_rows = stmt
        .query_map([], |row| {
            Ok(User {
                name: row.get(0)?,
                email: row.get(1)?,
                password: row.get(2)?,
                created_at: row.get(3)?,
                updated_at: row.get(4)?,
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

#[derive(Deserialize, Serialize, Debug)]
struct CollectionRequest {
    collection: String,
    fields: Vec<CollectionFields>,
}

#[derive(Deserialize, Serialize, Debug)]
struct CollectionFields {
    title: String,
    #[serde(rename = "type")]
    field_type: String,
    unique: bool,
    nullable: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    min: Option<u32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    max: Option<u32>,
}

async fn create_collection(data: web::Json<CollectionRequest>) -> Result<impl Responder> {
    let db_name = format!("databases/{}.db", data.collection);
    if data.collection.is_empty() {
        return Ok(HttpResponse::BadRequest().json(ApiResponse {
            message: "Collection name is required".to_string(),
            status: 409,
        }));
    }
    if Path::new(&db_name).exists() {
        return Ok(HttpResponse::BadRequest().json(ApiResponse {
            message: format!("Collection {} already exists!", &data.collection),
            status: 409,
        }));
    }

    let conn = match Connection::open(&db_name) {
        Ok(conn) => conn,
        Err(err) => {
            return Ok(HttpResponse::InternalServerError().json(ApiResponse {
                message: format!("Failed to create database: {}", err),
                status: 500,
            }));
        }
    };
    let mut create_table_sql = format!(
        "CREATE TABLE {} (id INTEGER PRIMARY KEY AUTOINCREMENT",
        data.collection
    );

    for field in &data.fields {
        let mut field_def = format!(
            ", {} {}",
            field.title,
            sql_type_from_field_type(&field.field_type)
        );

        if !field.nullable {
            field_def.push_str(" NOT NULL");
        }

        if field.unique {
            field_def.push_str(" UNIQUE");
        }

        if let Some(min) = field.min {
            if field.field_type == "VARCHAR" || field.field_type == "TEXT" {
                field_def.push_str(&format!(" CHECK(length({}) >= {})", field.title, min));
            } else if field.field_type == "INTEGER" || field.field_type == "FLOAT" {
                field_def.push_str(&format!(" CHECK({} >= {})", field.title, min));
            }
        }

        if let Some(max) = field.max {
            if field.field_type == "VARCHAR" || field.field_type == "TEXT" {
                field_def.push_str(&format!(" CHECK(length({}) <= {})", field.title, max));
            } else if field.field_type == "INTEGER" || field.field_type == "FLOAT" {
                field_def.push_str(&format!(" CHECK({} <= {})", field.title, max));
            }
        }

        create_table_sql.push_str(&field_def);
    }

    create_table_sql.push_str(")");

    if let Err(err) = conn.execute(&create_table_sql, []) {
        return Ok(HttpResponse::InternalServerError().json(ApiResponse {
            message: format!("Failed to create table: {}", err),
            status: 500,
        }));
    }

    let metadata_table = format!("{}_metadata", data.collection);
    let create_metadata_sql = format!(
        "CREATE TABLE {} (
            field_name TEXT PRIMARY KEY,
            field_type TEXT NOT NULL,
            unique_field BOOLEAN NOT NULL,
            nullable BOOLEAN NOT NULL,
            min INTEGER,
            max INTEGER
        )",
        metadata_table
    );

    if let Err(err) = conn.execute(&create_metadata_sql, []) {
        return Ok(HttpResponse::InternalServerError().json(ApiResponse {
            message: format!("Failed to create metadata table: {}", err),
            status: 500,
        }));
    }

    for field in &data.fields {
        let insert_metadata_sql = format!(
            "INSERT INTO {} (field_name, field_type, unique_field, nullable, min, max) 
             VALUES (?, ?, ?, ?, ?, ?)",
            metadata_table
        );

        if let Err(err) = conn.execute(
            &insert_metadata_sql,
            rusqlite::params![
                field.title,
                field.field_type,
                field.unique,
                field.nullable,
                field.min,
                field.max
            ],
        ) {
            return Ok(HttpResponse::InternalServerError().json(ApiResponse {
                message: format!("Failed to save field metadata: {}", err),
                status: 500,
            }));
        }
    }

    Ok(HttpResponse::Ok().json(ApiResponse {
        message: format!("Collection {} has been created!", &data.collection),
        status: 201,
    }))
}

fn sql_type_from_field_type(field_type: &str) -> &str {
    match field_type {
        "VARCHAR" => "TEXT",
        "TEXT" => "TEXT",
        "CHAR" => "TEXT",
        "CLOB" => "TEXT",
        "STRING" => "TEXT",
        "INTEGER" => "INTEGER",
        "INT" => "INTEGER",
        "SMALLINT" => "INTEGER",
        "BIGINT" => "INTEGER",
        "TINYINT" => "INTEGER",
        "MEDIUMINT" => "INTEGER",
        "FLOAT" => "REAL",
        "REAL" => "REAL",
        "DOUBLE" => "REAL",
        "DECIMAL" => "REAL",
        "NUMERIC" => "REAL",
        "BOOLEAN" => "INTEGER",
        "BOOL" => "INTEGER",
        "DATE" => "TEXT",
        "DATETIME" => "TEXT",
        "TIMESTAMP" => "TEXT",
        "TIME" => "TEXT",
        "BLOB" => "BLOB",
        "BINARY" => "BLOB",
        "VARBINARY" => "BLOB",
        _ => "TEXT",
    }
}
