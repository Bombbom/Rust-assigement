
///////////////////////////////////////////
// BAI 3
// Yêu cầu :
// + Sửa lại code sao cho compile cho đúng 
///////////////////////////////////////////


fn main() {
    let a = A {p: Some("p".to_string())};
    a.a();
}

struct A {
    p: Option<String>
}


impl A {
    fn a(self) -> Self {
        {
            let c = self.p.as_ref().unwrap();
            Self::b(c);
        }
        
        self
  
    }
    fn b(b: &str) {
        print!("b: {}", b)
    }
}