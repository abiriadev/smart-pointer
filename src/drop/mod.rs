struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("disallocate data of CustomSmartPointer: {}", self.data);
    }
}

pub fn drop_main() {
    let c = CustomSmartPointer {
        data: String::from("my data"),
    };
    let d = CustomSmartPointer {
        data: String::from("other data"),
    };

    drop(c);
    println!("STARTING");
}
