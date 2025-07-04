pub fn hello_world() {
    println!("Hello from cmdr-core!");

    // TODO: wire up llama.cpp FFI bindings in Phase 2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hello_world() {
        hello_world();
    }
}
