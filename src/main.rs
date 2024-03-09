use actix_web::{web, App, HttpServer, Responder, HttpResponse};
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};
use std::collections::HashMap;
use std::fs::File;
use std::io::Write;
use chrono::NaiveDate;
use chrono::prelude::*;
use std::sync::atomic::AtomicU32;
use std::sync::atomic::Ordering;

static FILE_COUNTER: AtomicU32 = AtomicU32::new(0);

#[derive(Debug, Serialize, Deserialize, Clone)]
struct Client {
    id: u32,
    name: String,
    birth_date: NaiveDate,
    document_number: String,
    country: String,
    balance: f64,
}

#[derive(Debug, Serialize, Deserialize)]
struct NewClient {
    client_name: String,
    birth_date: NaiveDate,
    document_number: String,
    country: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct CreditTransaction {
    client_id: u32,
    debit_mount: f64,
}

struct AppState {
    clients: Arc<Mutex<HashMap<u32, Client>>>,
}


 // Lógica para crear un nuevo cliente y devolver su ID
 async fn new_client(new_client: web::Json<NewClient>, state: web::Data<AppState>) -> impl Responder {
    let mut clients = state.clients.lock().unwrap();

    // Verifica si el número de documento ya existe en la base de datos
    let document_number = &new_client.document_number;
    if clients.values().any(|client| client.document_number == *document_number) {
        return HttpResponse::BadRequest().body("El número de documento ya está registrado");
    }

    // Genera un nuevo ID para el cliente
    let next_id = clients.len() as u32 + 1;

    // Crea el nuevo cliente
    let new_client_data = new_client.into_inner();
    let new_client = Client {
        id: next_id,
        name: new_client_data.client_name,
        birth_date: new_client_data.birth_date,
        document_number: new_client_data.document_number,
        country: new_client_data.country,
        balance: 0.0,
    };

    clients.insert(next_id, new_client.clone());

    HttpResponse::Ok().json(next_id)
}


async fn new_credit_transaction(credit_transaction: web::Json<CreditTransaction>, state: web::Data<AppState>) -> impl Responder {
    let transaction_data = credit_transaction.into_inner();
    let client_id = transaction_data.client_id;
    let debit_amount = transaction_data.debit_mount;

    let mut clients = state.clients.lock().unwrap();

    // Verifica si el cliente existe en la base de datos
    let client = match clients.get_mut(&client_id) {
        Some(client) => client,
        None => return HttpResponse::BadRequest().body("El cliente no existe"),
    };

    if debit_amount < 0.0 {
        return HttpResponse::BadRequest().body("El monto de la transacción debe ser positivo");
    }

    client.balance += debit_amount;

    HttpResponse::Ok().json(client.balance)
}


async fn store_balances(state: web::Data<AppState>) -> impl Responder {
 // Incrementa el contador de archivo
 let file_counter = FILE_COUNTER.fetch_add(1, Ordering::SeqCst);

 // Genera el nombre del archivo con el nuevo contador
 let file_name = generate_file_name(file_counter);

 // obtiene el estado de la aplicación
 let clients = state.clients.lock().unwrap();

 // Abre o crea el archivo
 let mut file = match File::create(&file_name) {
     Ok(file) => file,
     Err(_) => return HttpResponse::InternalServerError().body("Error al crear el archivo"),
 };

 // Escribe los saldos de los clientes en el archivo
 for (client_id, client) in clients.iter() {
     if let Err(_) = writeln!(file, "{}: {}", client_id, client.balance) {
         return HttpResponse::InternalServerError().body("Error al escribir en el archivo");
     }
 }

 HttpResponse::Ok().body(format!("Saldos almacenados en el archivo: {}", file_name))
}



fn generate_file_name(counter: u32) -> String {
    // Obtiene la fecha actual
    let current_date = Local::now();
    let formatted_date = current_date.format("%d%m%Y").to_string();

    // Concatena la fecha y el contador para formar el nombre del archivo
    let file_name = format!("{}_{}.DAT", formatted_date, counter);

    file_name
}

async fn client_balance(path: web::Path<u32>, state: web::Data<AppState>) -> impl Responder {
    let client_id = path.into_inner();
    let clients = state.clients.lock().unwrap();

    // Busca al cliente por su ID
    if let Some(client) = clients.get(&client_id) {
        HttpResponse::Ok().json(client.balance)
    } else {
        HttpResponse::NotFound().body("Cliente no encontrado")
    }
}


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let clients: HashMap<u32, Client> = HashMap::new();
    let state = Arc::new(Mutex::new(clients));

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(AppState { clients: state.clone() }))
            .route("/new_client", web::post().to(new_client))
            .route("/new_credit_transaction", web::post().to(new_credit_transaction))
            .route("/store_balances", web::post().to(store_balances))
            .route("/client_balance/{client_id}", web::get().to(client_balance))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}