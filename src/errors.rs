use validator::{Validate, ValidationError, ValidationErrors};

#[derive(Debug)]
pub struct Errors {
    errors: ValidationErrors,
}

pub type FieldName = &'static str;
pub type FieldErrorCode = &'static str;

pub struct FieldValidator {
    errors: ValidationErrors,
}

impl Errors {
    pub fn new(errs: &[(FieldName, FieldErrorCode)]) -> Self {
        let mut errors = ValidationErrors::new();
        for (field, code) in errs {
            errors.add(field, ValidationError::new(code));
        }
        Self { errors }
    }
}

impl Default for FieldValidator {
    fn default() -> Self {
        Self {
            errors: ValidationErrors::new(),
        }
    }
}

impl FieldValidator {
    pub fn validate<T: Validate>(model: &T) -> Self {
        Self {
            errors: model.validate().err().unwrap_or_else(ValidationErrors::new),
        }
    }

    /// Convenience method to trigger early returns with ? operator.
    pub fn check(self) -> Result<(), Errors> {
        if self.errors.is_empty() {
            Ok(())
        } else {
            Err(Errors {
                errors: self.errors,
            })
        }
    }

    pub fn extract<T>(&mut self, field_name: &'static str, field: Option<T>) -> T
        where
            T: Default,
    {
        field.unwrap_or_else(|| {
            self.errors
                .add(field_name, ValidationError::new("can't be blank"));
            T::default()
        })
    }
}