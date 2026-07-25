use axum::{Json, extract::{FromRequest, Request, rejection::JsonRejection}, http::StatusCode};
use serde::{Deserialize, Serialize};
use serde_json::{Map, Value, json};

#[derive(Deserialize, Debug, Serialize)]
#[serde(untagged)]
pub enum RpcId {
    String(String),
    Number(i64),
    Null,
}

#[derive(Deserialize, Debug)]
#[serde(untagged)]
pub enum RpcParams {
    Array(Vec<Value>),
    Object(Map<String, Value>),
}

#[derive(Deserialize, Debug)]
pub struct RpcRequest {
    pub id: Option<RpcId>,
    pub params: Option<RpcParams>,
    pub method: String,

    #[serde(rename = "jsonrpc")]
    pub jsonrpc: String,
}


// --------------------------------------------------
// Custom extractor
// --------------------------------------------------




pub struct JsonRpc(pub RpcRequest);

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