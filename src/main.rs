

use moro_local::async_scope;

// #[tokio::main(flavor = "current_thread")]
pub fn main() {
    let rt = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .expect("build runtime");

    rt.block_on(async {
        let mut container = vec![1, 2, 3];
        let mut num = 0;
        println!("hello from the main future");
        async_scope!(|scope| {
            let a = async {
                println!("hello from the first future");
                dbg!(&container);
            };

            let b = async {
                println!("hello from the second future");
                num += container[0] + container[2];
            };
            scope.spawn(a);
            scope.spawn(b);
        }).await;

        
        container.push(4);
        assert_eq!(num, container.len());
        println!("container: {:?}", container);
        println!("num: {:?}", num);
    });
}
