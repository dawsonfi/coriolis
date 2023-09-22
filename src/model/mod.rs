use utoipa::ToSchema;

#[derive(Debug, PartialEq, Eq, ToSchema)]
pub struct ParseEnumError {
    pub message: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_enum_error_should_be_thread_safe() {
        is_thread_safe::<ParseEnumError>();
    }

    fn is_thread_safe<T: Sized + Send + Sync + Unpin>() {}
}
