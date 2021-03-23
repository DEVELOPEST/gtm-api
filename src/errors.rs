use rocket::{Request, response};
use rocket::http::Status;
use rocket::response::{Responder, status};
use rocket_contrib::json::Json;
use validator::{Validate, ValidationError};

#[derive(Debug)]
pub enum Error {
    ValidationError(ValidationErrors),
    DatabaseError(diesel::result::Error),
    AuthorizationError(&'static str),
    HttpError(reqwest::Error),
    BadRequest(&'static str),
    Custom(&'static str),
}

impl<'a> Responder<'a> for Error {
    fn respond_to(self, req: &Request) -> response::Result<'a> {
        match self {
            Error::ValidationError(err) => {
                use validator::ValidationErrorsKind::Field;
                let mut errors = json!({});
                for (field, field_errors) in err.errors.into_errors() {
                    if let Field(field_errors) = field_errors {
                        errors[field] = field_errors.into_iter()
                            .map(|field_error| field_error.code)
                            .collect();
                    }
                }
                status::Custom(
                    err.status,
                    Json(json!({ "errors": errors })),
                ).respond_to(req)
            }
            Error::DatabaseError(err) => {
                error!("{}", err);
                status::Custom(
                    Status::InternalServerError,
                    Json(json!({ "error": "Something went wrong! :(" })),
                ).respond_to(req)
            }
            Error::AuthorizationError(err) => {
                status::Custom(
                    Status::Unauthorized,
                    Json(json!({ "error" : err }))
                ).respond_to(req)
            }
            Error::HttpError(err) => {
                error!("{}", err);
                status::Custom(
                    Status::FailedDependency,
                    Json(json!({ "error": "Some API request failed, try again later!" })),
                ).respond_to(req)
            }
            Error::BadRequest(msg) => {
                status::BadRequest(
                    Option::from(Json(json!({ "error": msg })))
                ).respond_to(req)
            }
            Error::Custom(msg) => {
                status::Custom(
                    Status::InternalServerError,
                    Json(json!({ "error": msg }))
                ).respond_to(req)
            }
        }
    }
}

#[derive(Debug)]
pub struct ValidationErrors {
    status: Status,
    errors: validator::ValidationErrors,
}

impl From<ValidationErrors> for Error {
    fn from(err: ValidationErrors) -> Self {
        Error::ValidationError(err)
    }
}

impl From<diesel::result::Error> for Error {
    fn from(err: diesel::result::Error) -> Self {
        Error::DatabaseError(err)
    }
}

impl From<reqwest::Error> for Error {
    fn from(err: reqwest::Error) -> Self {
        Error::HttpError(err)
    }
}

pub type FieldName = &'static str;
pub type FieldErrorCode = &'static str;

pub struct FieldValidator {
    errors: validator::ValidationErrors,
}

impl ValidationErrors {
    pub fn new(errs: &[(FieldName, FieldErrorCode)], resp_status: Option<Status>) -> Self {
        let mut errors = validator::ValidationErrors::new();
        for (field, code) in errs {
            errors.add(field, ValidationError::new(code));
        }
        Self { status: resp_status.unwrap_or(Status::InternalServerError), errors }
    }
}

impl Default for FieldValidator {
    fn default() -> Self {
        Self {
            errors: validator::ValidationErrors::new(),
        }
    }
}

impl FieldValidator {
    pub fn validate<T: Validate>(model: &T) -> Self {
        Self {
            errors: model.validate().err().unwrap_or_else(validator::ValidationErrors::new),
        }
    }

    /// Convenience method to trigger early returns with ? operator.
    pub fn check(self) -> Result<(), Error> {
        if self.errors.is_empty() {
            Ok(())
        } else {
            Err(Error::ValidationError(
                ValidationErrors {
                    status: Status::UnprocessableEntity,
                    errors: self.errors,
                }))
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
        if !(interval == "hour" || interval == "day" || interval == "week" || interval == "month" || interval == "year")  {
            self.errors
                .add("interval", ValidationError::new("Invalid interval!"));
        }
    }
}