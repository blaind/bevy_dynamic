use bevy::{prelude::*, tasks::AsyncComputeTaskPool};

pub fn test_system(thread_pool: Res<AsyncComputeTaskPool>) {
    thread_pool.spawn(async move {
        async_compat::Compat::new(async {
            do_request().await;
        })
        .await;
    });
}

async fn do_request() {
    let req = reqwest::get("http://localhost:12345").await;
    println!("Request: {:?}", req);
}
