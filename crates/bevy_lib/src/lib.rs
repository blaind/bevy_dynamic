use bevy::{prelude::*, tasks::AsyncComputeTaskPool};

pub fn test_system(mut commands: Commands, thread_pool: Res<AsyncComputeTaskPool>) {
    let task = thread_pool.spawn(async move {
        async_compat::Compat::new(async {
            do_request().await;
        })
        .await;
    });

    commands.spawn().insert(task);
}

async fn do_request() {
    println!("Doing request...");
    let req = reqwest::get("http://localhost:12345").await;
    println!("Request success: {:?}", req.is_ok());
}
