use serde::Serialize;

#[derive(Serialize)]
pub struct GenericResponse {
    pub status_code: u16,
    pub message: String,
}

#[derive(Serialize)]
pub struct TypedResponse<T> {
    pub body: T,
    pub response: GenericResponse,
}