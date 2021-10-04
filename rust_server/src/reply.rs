use crate::prelude::*;
use rocket::{
    response::{self, Responder},
    serde::json::Json,
    Request,
};

#[derive(Debug, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum Reply<T>
where
    T: Serialize,
{
    Ok(T),
    #[allow(dead_code)]
    ClientError(String),
    #[allow(dead_code)]
    ServerError,
}

impl<'r, 'o: 'r, T> Responder<'r, 'o> for Reply<T>
where
    T: Serialize,
{
    fn respond_to(self, req: &'r Request) -> response::Result<'o> {
        Json(self).respond_to(req)
    }
}
