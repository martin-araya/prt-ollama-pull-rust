use actix_web::{post, web, App, HttpResponse, HttpServer, Responder};
use serde::{Deserialize, Serialize};
use reqwest::Client;
use serde_json::json;
use futures::StreamExt;

#[derive(Deserialize)]
struct PullRequest {
    name: String,
    insecure: Option<bool>,
    stream: Option<bool>,
}

#[derive(Serialize, Deserialize)]
struct PullResponse {
    message: String,
}

#[post("/api/pull")]
async fn pull_model(req: web::Json<PullRequest>) -> impl Responder {
    let client = Client::new();
    let url = "http://localhost:11434/api/pull";

    let body = json!({
        "name": req.name,
        "insecure": req.insecure.unwrap_or(false),
        "stream": req.stream.unwrap_or(true),
    });

    match client.post(url).json(&body).send().await {
        Ok(response) => {
            if req.stream.unwrap_or(true) {
                let mut stream = response.bytes_stream();

                // Crear un stream de respuesta que enviará los datos en tiempo real
                let response_stream = async_stream::stream! {
                    while let Some(chunk) = stream.next().await {
                        match chunk {
                            Ok(bytes) => {
                                // Enviar cada fragmento de datos tan pronto como esté disponible
                                yield Ok::<_, actix_web::Error>(web::Bytes::from(bytes));
                            }
                            Err(_) => {
                                // En caso de error, enviar un mensaje de error al cliente
                                yield Err(actix_web::error::ErrorInternalServerError("Error reading stream"));
                            }
                        }
                    }
                };

                // Configurar la respuesta para el streaming de progreso
                return HttpResponse::Ok()
                    .content_type("application/octet-stream") // Tipo de contenido de streaming
                    .streaming(response_stream);
            }

            // Si `stream` es `false`, manejamos la respuesta como un único JSON
            match response.json::<PullResponse>().await {
                Ok(json) => HttpResponse::Ok().json(json),
                Err(_) => HttpResponse::InternalServerError().body("Error parsing Ollama response"),
            }
        }
        Err(_) => HttpResponse::InternalServerError().body("Failed to connect to Ollama"),
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(pull_model) // Registra el endpoint
    })
        .bind("127.0.0.1:8081")?
        .run()
        .await
}
