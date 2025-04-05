#[cfg(test)]
mod tests {
    use rtop::utils;

    #[test]
    fn bytes_to_gb() {
        let gbs = utils::helpers::byte_to_readable_size(8589934592);
        assert_eq!(gbs, (8.0, String::from("GiB")));
    }
}
