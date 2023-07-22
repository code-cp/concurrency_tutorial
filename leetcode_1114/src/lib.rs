pub struct Foo; 

impl Foo {
    pub async fn first(&self) {
        println!("first");
    }

    pub async fn second(&self) {
        println!("second"); 
    }

    pub async fn third(&self) {
        println!("thrid"); 
    }
}

