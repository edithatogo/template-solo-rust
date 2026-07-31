pub fn ready() -> bool {
    true
}

#[cfg(test)]
mod tests {
    use super::ready;

    #[test]
    fn smoke() {
        assert!(ready());
    }
}