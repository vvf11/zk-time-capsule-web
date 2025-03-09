use actix_web::{post, web, App, HttpResponse, HttpServer, Responder};
use serde::{Deserialize, Serialize};
use sp1_sdk::{ProverClient, SP1Stdin};

#[derive(Deserialize)]
struct ProofRequest {
    unlock_date: u64,
    message: String,
    photo_data: Vec<u8>,
}

#[derive(Serialize)]
struct ProofResponse {
    proof: Vec<u8>,
}

#[post("/generate_proof")]
async fn generate_proof_endpoint(req: web::Json<ProofRequest>) -> impl Responder {
    let current_date = 20250309;

    if current_date >= req.unlock_date {
        return HttpResponse::BadRequest().body("Дата открытия должна быть в будущем!");
    }

    let mut stdin = SP1Stdin::new();
    stdin.write(¤t_date);
    stdin.write(&req.unlock_date);
    stdin.write(&req.message);
    stdin.write(&req.photo_data);

    let client = ProverClient::new();
    match client.prove(&stdin) {
        Ok(proof) => HttpResponse::Ok().json(ProofResponse { proof }),
        Err(e) => HttpResponse::InternalServerError().body(format!("Ошибка: {:?}", e)),
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(generate_proof_endpoint)
            .service(actix_files::Files::new("/", "./public").index_file("index.html"))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
