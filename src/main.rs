

// #[tokio::main(flavor = "current_thread")]
pub fn main() {
    let rt = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .expect("build runtime");

    rt.block_on(async {
        println!("hello from the main future");
        let a = async {
            let mut count = 5;
            while count > 0 {
                println!("hello from the first future");
                tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
                count -= 1;
            }
            Ok::<(), String>(())

        };

        let b = async {
            let mut count = 3;
            while count > 0 {
                println!("hello from the second future");
                tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
                count -= 1;
            }
            Err::<(), String>("Second Future Error".to_string())
        };

        let (a, b) = futures::join!(a, b);
        a.unwrap();

    });
}
