use deref::{DerefMut, deref_mut};

#[derive(Debug, DerefMut)]
struct Hello<T> {
    #[deref_mut]
    inner: T,
}

#[derive(Debug)]
struct Hello2<T: Send> {
    inner: Hello<T>,
}

deref_mut!(<T: Send>, Hello2<T>, T, inner);

fn main() {
    let mut hello = Hello {
        inner: "Hello, World".to_string(),
    };

    println!("{:?}", hello);

    *hello += "!";

    println!("{:?}", hello);

    let mut hello2 = Hello2 {
        inner: Hello {
            inner: "Hello, World 2".to_string(),
        },
    };

    println!("{:?}", hello2);

    *hello2 += "!";

    println!("{:?}", hello2);
}
