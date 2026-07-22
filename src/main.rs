use axum::{
    extract::{
        rejection::JsonRejection,
        FromRequest,
        Request,
    },
    http::StatusCode,
    routing::post,
    Json, Router,
};
use serde::Deserialize;
use serde_json::{json, Map, Value};

#[derive(Deserialize, Debug)]
#[serde(untagged)]
enum RpcId {
    String(String),
    Number(i64),
    Null,
}

#[derive(Deserialize, Debug)]
#[serde(untagged)]
enum RpcParams {
    Array(Vec<Value>),
    Object(Map<String, Value>),
}

#[derive(Deserialize, Debug)]
struct RpcRequest {
    id: Option<RpcId>,
    params: Option<RpcParams>,
    method: String,

    #[serde(rename = "jsonrpc")]
    jsonrpc: String,
}

// --------------------------------------------------
// Custom extractor
// --------------------------------------------------

struct JsonRpc(RpcRequest);

type Rejection = (StatusCode, Json<Value>);

//#[async_trait]
impl<S> FromRequest<S> for JsonRpc
where
    S: Send + Sync,
{
    type Rejection = Rejection;

    async fn from_request(
        req: Request,
        state: &S,
    ) -> Result<Self, Self::Rejection> {
        match Json::<RpcRequest>::from_request(req, state).await {
            // Successfully parsed JSON
            Ok(Json(data)) => {
                Ok(JsonRpc(data))
            }

            // JSON parsing failed
            Err(rejection) => match rejection {
                JsonRejection::MissingJsonContentType(_) => Err((
                    StatusCode::UNSUPPORTED_MEDIA_TYPE,
                    Json(json!({
                        "jsonrpc": "2.0",
                        "id": null,
                        "error" : {
                            "code": -32600,
                            "message" : "Invalid request"
                        }

                    }))
                    //"Content-Type must be application/json",
                )),

                JsonRejection::JsonSyntaxError(_) => Err((
                    StatusCode::BAD_REQUEST,
                    Json(json!({
                        "jsonrpc": "2.0",
                        "id": null,
                        "error": {
                            "code": -32700,
                            "message": "Parse error"
                        }
                    }))
                    //"Malformed JSON",
                )),

                JsonRejection::JsonDataError(_) => Err((
                    StatusCode::BAD_REQUEST,
                    Json(json!({
                        "jsonrpc": "2.0",
                        "id": null,
                        "error": {
                            "code": -32600,
                            "message": "Parse error"
                        }
                    }))
                    //"JSON has invalid structure",
                )),

                _ => Err((
                    StatusCode::BAD_REQUEST,
                    Json(json!({
                        "jsonrpc": "2.0",
                        "id": null,
                        "error": {
                            "code": -32600,
                            "message": "Parse error"
                        }
                    }))
                    //"Invalid request",
                )),
            },
        }
    }
}

// --------------------------------------------------
// Handler
// --------------------------------------------------

async fn rpc(JsonRpc(payload): JsonRpc) -> Json<Value> {
    println!("{:#?}", payload);

    Json(json!({
        "status": "ok"
    }))
}

// --------------------------------------------------
// Main
// --------------------------------------------------

#[tokio::main]
async fn main() {
    let app = Router::new().route("/rpc", post(rpc));

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();

    println!("Listening on http://127.0.0.1:3000");

    axum::serve(listener, app)
        .await
        .unwrap();
}