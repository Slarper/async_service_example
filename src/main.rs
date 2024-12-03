use futures_concurrency::prelude::*;



// #[tokio::main(flavor = "current_thread")]
pub fn main() {
let rt = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .expect("build runtime");

    rt.block_on(async move {

        let mut container = vec![1, 2, 3];
        let mut num = 0;

        let a = async {
            println!("hello from the first future");
            dbg!(&container);
        };

        let b = async {
            println!("hello from the second future");
            num += container[0] + container[2];
        };

        println!("hello from the main future");
        let _ = (a, b).join().await;
        container.push(4);
        assert_eq!(num, container.len());
    })
}
