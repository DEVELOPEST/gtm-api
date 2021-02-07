use rocket::{Request, response};
use rocket::http::Status;
use rocket::response::{Responder, status};
use rocket_contrib::json::Json;
use validator::{Validate, ValidationError, ValidationErrors};

#[derive(Debug)]
pub struct Errors {
    status: Status,
    errors: ValidationErrors,
}

impl<'a> Responder<'a> for Errors {
    fn respond_to(self, req: &Request) -> response::Result<'a> {
        use validator::ValidationErrorsKind::Field;

        let mut errors = json!({});
        for (field, field_errors) in self.errors.into_errors() {
            if let Field(field_errors) = field_errors {
                errors[field] = field_errors.into_iter().map(|field_error| field_error.code).collect();
            }
        }

        status::Custom(
            self.status,
            Json(json!({ "errors": errors })),
        ).respond_to(req)
    }
}

pub type FieldName = &'static str;
pub type FieldErrorCode = &'static str;

pub struct FieldValidator {
    errors: ValidationErrors,
}

impl Errors {
    pub fn new(errs: &[(FieldName, FieldErrorCode)], resp_status: Option<Status>) -> Self {
        let mut errors = ValidationErrors::new();
        for (field, code) in errs {
            errors.add(field, ValidationError::new(code));
        }
        Self { status: resp_status.unwrap_or(Status::InternalServerError), errors }
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
                status: Status::UnprocessableEntity,
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

    pub fn validate_timeline_period(
        &mut self,
        start: i64,
        end :i64,
        interval: &str)
    {
        if start < 0 || start > end {
            self.errors
                .add("period", ValidationError::new("Invalid period!"));
        }

        if end - start > (370 * 24 * 60 * 60) { // little over year
            self.errors
                .add("period", ValidationError::new("Too long period!"));
        }

        let interval = &*interval.to_lowercase();
        if !(interval == "hour" || interval == "day" || interval == "week" || interval == "month")  {
            self.errors
                .add("interval", ValidationError::new("Invalid interval!"));
        }
    }
}