Documentación del Código para el Endpoint /api/pull
Este documento detalla cada parte del código para el endpoint /api/pull en Actix Web, que realiza una solicitud POST al servicio Ollama para descargar un modelo y muestra el progreso en tiempo real.

1. Estructuras de Datos: PullRequest y PullResponse
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
PullRequest: Define la estructura de la solicitud con los siguientes campos:
name: Nombre del modelo a descargar.
insecure: Permite conexiones inseguras; opcional.
stream: Controla si la descarga debe ser en tiempo real; opcional.
PullResponse: Estructura de respuesta que incluye un mensaje de confirmación.
Deriva de Serialize y Deserialize para convertir entre JSON y Rust.


Aquí tienes el contenido reestructurado en formato Markdown para el README:

Documentación del Código para el Endpoint /api/pull
Este documento detalla cada parte del código para el endpoint /api/pull en Actix Web, que realiza una solicitud POST al servicio Ollama para descargar un modelo y muestra el progreso en tiempo real.

1. Estructuras de Datos: PullRequest y PullResponse
   rust
   Copy code
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
PullRequest: Define la estructura de la solicitud con los siguientes campos:
name: Nombre del modelo a descargar.
insecure: Permite conexiones inseguras; opcional.
stream: Controla si la descarga debe ser en tiempo real; opcional.
PullResponse: Estructura de respuesta que incluye un mensaje de confirmación.
Deriva de Serialize y Deserialize para convertir entre JSON y Rust.
2. Definición del Endpoint pull_model
   rust
   Copy code
   #[post("/api/pull")]
   async fn pull_model(req: web::Json<PullRequest>) -> impl Responder {
   #[post("/api/pull")]: Define este endpoint como una solicitud POST en /api/pull.
   pull_model: Función que maneja la solicitud de descarga del modelo.
   req: web::Json<PullRequest>: Actix Web convierte automáticamente la solicitud JSON entrante en una instancia de PullRequest, permitiendo acceso a los campos name, insecure, y stream.
3. Preparación de la Solicitud a Ollama
   rust
   Copy code
   let client = Client::new();
   let url = "http://localhost:11434/api/pull";

let body = json!({
"name": req.name,
"insecure": req.insecure.unwrap_or(false),
"stream": req.stream.unwrap_or(true),
});
Client::new(): Crea un cliente HTTP de reqwest para enviar la solicitud a Ollama.
url: Define la URL del endpoint de Ollama.
body: Crea el cuerpo JSON de la solicitud con los campos de PullRequest. Usa unwrap_or para definir valores predeterminados:
insecure predeterminado a false.
stream predeterminado a true.
4. Enviar la Solicitud y Manejar la Respuesta
   rust
   Copy code
   match client.post(url).json(&body).send().await {
   Ok(response) => {
   if req.stream.unwrap_or(true) {
   let mut stream = response.bytes_stream();
   client.post(url).json(&body).send().await: Envia la solicitud POST a Ollama con el cuerpo JSON.
   Si la solicitud es exitosa (Ok(response)), se procesa.
   response.bytes_stream(): Obtiene los datos en tiempo real (modo stream) si stream es true.
5. Procesar el Stream de Progreso en Tiempo Real
   rust
   Copy code
   let response_stream = async_stream::stream! {
   while let Some(chunk) = stream.next().await {
   match chunk {
   Ok(bytes) => {
   yield Ok::<_, actix_web::Error>(web::Bytes::from(bytes));
   }
   Err(_) => {
   yield Err(actix_web::error::ErrorInternalServerError("Error reading stream"));
   }
   }
   }
   };
   async_stream::stream!: Crea un stream asíncrono en el que cada fragmento de datos (chunk) se envía al cliente en tiempo real.
   while let Some(chunk) = stream.next().await: Lee cada fragmento de bytes a medida que Ollama lo envía.
   yield Ok::<_, actix_web::Error>(web::Bytes::from(bytes)): Envía cada fragmento al cliente en formato Bytes.
   yield Err(...): Si ocurre un error al leer el stream, envía un mensaje de error al cliente.
6. Configurar la Respuesta para Streaming
   rust
   Copy code
   return HttpResponse::Ok()
   .content_type("application/octet-stream")
   .streaming(response_stream);
   HttpResponse::Ok().content_type("application/octet-stream"): Envía una respuesta 200 con tipo de contenido octet-stream, adecuado para datos binarios.
   .streaming(response_stream): Envía el response_stream en tiempo real al cliente.
7. Manejar la Respuesta Completa (si stream es false)
   rust
   Copy code
   match response.json::<PullResponse>().await {
   Ok(json) => HttpResponse::Ok().json(json),
   Err(_) => HttpResponse::InternalServerError().body("Error parsing Ollama response"),
   }
   response.json::<PullResponse>().await: Si stream es false, deserializa la respuesta completa en PullResponse.
   HttpResponse::Ok().json(json): Envía la respuesta JSON completa al cliente.
8. Manejo de Errores en la Solicitud a Ollama
   rust
   Copy code
   Err(_) => HttpResponse::InternalServerError().body("Failed to connect to Ollama"),
   Err(_): Si no se puede conectar a Ollama, envía una respuesta de error 500 al cliente.
   Configuración y Ejecución del Servidor Actix Web
   rust
   Copy code
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
   #[actix_web::main]: Convierte la función main en un runtime asíncrono para Actix Web.
   HttpServer::new: Crea un nuevo servidor HTTP Actix.
   App::new().service(pull_model): Registra el endpoint pull_model en el servidor.
   .bind("127.0.0.1:8081"): Configura el servidor para escuchar en localhost:8081.
   .run().await: Inicia el servidor y espera a que se apaguen todos los servicios.

Este documento proporciona una descripción detallada del código para el endpoint /api/pull en Actix Web, incluyendo las estructuras de datos, la definición del endpoint, la preparación y envío de la solicitud a Ollama, el procesamiento del stream en tiempo real, la configuración de la respuesta para streaming, el manejo de la respuesta completa, y el manejo de errores en la solicitud a Ollama. También se incluye la configuración y ejecución del servidor Actix Web para el endpoint.