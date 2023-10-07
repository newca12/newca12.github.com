#[cfg(test)]
mod tests {
    use gallery_rs_utils::run_example;

    #[test]
    fn futures() {
        run_example("futures");
    }

    #[test]
    fn tokio() {
        run_example("tokio");
    }

    #[test]
    fn tokio_stream() {
        run_example("tokio_stream");
    }
}
