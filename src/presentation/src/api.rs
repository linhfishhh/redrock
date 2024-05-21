use std::rc::Rc;
use axum::extract::{Query, State};
use deadpool_postgres::Pool;
use crate::requests::FilterRequest;

pub async fn get_all(
    State(pool): State<Pool>,
    params: Option<Query<FilterRequest>>
){

}