pub fn main() {
    let rt = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .expect("build runtime");

    let a = rt.block_on(async move {
        println!("hello from the main future");

        moro_local::async_scope!(|scope| {
            let k = scope.spawn(async {
                for i in 0..10 {
                    //  tokio sleep 100 ms
                    tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
                    println!("hello from the second child future: {}", i);
                }
                514
            });
            let x = k.await;

            let h = scope.spawn(async {
                println!("hello 111");
                114
            });


            let y = h.await;

            x + y
        })
        .await
    });

    println!("a: {}", a);
}
