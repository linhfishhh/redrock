use std::ops::DerefMut;
use deadpool_postgres::Pool;
use tokio_postgres::Error;
use crate::migrations;

pub async fn configure(pool: &Pool) -> Result<(),Error> {
    let mut obj = pool.get().await.unwrap();
    let client = obj.deref_mut().deref_mut();

    migrations::migrations::runner()
        .run_async(client)
        .await
        .unwrap();
    Ok::<(), Error>(())
}