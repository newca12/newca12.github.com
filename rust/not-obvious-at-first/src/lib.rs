#[cfg(test)]
mod tests {
    use gallery_rs_utils::run_example;

    //tokio-vs-futures
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

    //spurious-stream-lock
    #[test]
    #[should_panic]
    fn string_stream() {
        run_example("string_stream");
    }

    #[test]
    #[should_panic]
    fn async_string_stream() {
        run_example("async_string_stream");
    }

    #[test]
    #[ignore]
    fn async_string_futures_stream_lock() {
        run_example("async_string_futures_stream_lock");
    }

    #[test]
    #[ignore]
    fn async_string_futures_gz_stream_lock() {
        run_example("async_string_futures_gz_stream_lock");
    }

    #[test]
    fn async_string_tokio_stream_nolock() {
        run_example("async_string_tokio_stream_nolock");
    }

    #[test]
    fn async_string_tokio_gz_stream_lock() {
        run_example("async_string_tokio_gz_stream_nolock");
    }
}
