#[cfg(test)]
mod tests {
    use crate::error::Error;

    #[test]
    fn error_variants_return_proper_messages() {
        let pairs: &[(Error, &str)] = &[
            (Error::CannotBuildPath, "cannot construct path"),
            (
                Error::CannotFindDir("parent".to_string()),
                "cannot find parent directory",
            ),
            (
                Error::CannotCreateDir("./til/notes".to_string()),
                "cannot create ./til/notes directory",
            ),
            (
                Error::CannotProcessArgs,
                "cannot process command-line arguments",
            ),
            ("custom message".into(), "custom message"),
            (Error::default(), "something wrong happened"),
        ];

        pairs
            .iter()
            .for_each(|(err, msg)| assert_eq!(err.message(), msg.to_string()));
    }
}
