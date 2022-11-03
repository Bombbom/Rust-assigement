
/////////////////////////////////////
// Bài 2
// Implement trait
// Thực hiên cargo test
/////////////////////////////////////


trait AppendBar {
    fn append_bar(self) -> Self;
}

impl AppendBar for String {
    fn append_bar(self) -> Self {
        let mut clone_self = self.clone();
        clone_self.push_str("Bar");
        clone_self
    }
    //Add your code here
}

fn main() {
    let s = String::from("Foo");
    let s = s.append_bar();
    println!("s: {}", s);
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_foo_bar() {
        assert_eq!(String::from("Foo").append_bar(), String::from("FooBar"));
    }

    #[test]
    fn is_bar_bar() {
        assert_eq!(
            String::from("").append_bar().append_bar(),
            String::from("BarBar")
        );
    }
}

// cargo test

// warning: crate `Assignment_12` should have a snake case name
//   |
//   = note: `#[warn(non_snake_case)]` on by default
//   = help: convert the identifier to snake case: `assignment_12`

// warning: `Assignment-12` (bin "Assignment-12" test) generated 1 warning
//     Finished test [unoptimized + debuginfo] target(s) in 0.30s
//      Running unittests src\main.rs (target\debug\deps\Assignment_12-dc2c98515dfd1391.exe)     

// running 2 tests
// test tests::is_bar_bar ... ok
// test tests::is_foo_bar ... ok

// test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
