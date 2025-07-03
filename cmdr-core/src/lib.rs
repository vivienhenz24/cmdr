pub fn hello_world() {
    println!("Hello from cmdr-core!");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hello_world() {
        hello_world();
    }
} 