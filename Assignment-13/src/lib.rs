
/////////////////////////////////////
// Bài 3
// Implement trait
// Thực hiên cargo test
/////////////////////////////////////



trait AppendBar {
    fn append_bar(self) -> Self;
}
//TODO: Add your code here

impl AppendBar for Vec<String> {
    fn append_bar(self) -> Self {
        let mut clone_self = self.clone();
        clone_self.push("Bar".to_owned());
        clone_self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_vec_pop_eq_bar() {
        let mut foo = vec![String::from("Foo")].append_bar();
        assert_eq!(foo.pop().unwrap(), String::from("Bar"));
        assert_eq!(foo.pop().unwrap(), String::from("Foo"));
    }
}

// running 1 test
// test tests::is_vec_pop_eq_bar ... ok

// test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
