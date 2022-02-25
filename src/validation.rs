pub trait Validate {
    type ValidationError;

    fn validate(&self) -> Result<(), Self::ValidationError>;
}
