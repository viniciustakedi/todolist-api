use actix_web::{http::StatusCode, HttpResponse, Responder};
use serde::Serialize;

#[derive(Serialize)]
pub struct CreateResponseText {
    message: String,
    status_code: u16,
}

pub async fn create_response_message(message: String, status_code: u16) -> impl Responder {
    let res: CreateResponseText = CreateResponseText {
        message,
        status_code,
    };

    HttpResponse::build(StatusCode::from_u16(status_code).unwrap()).json(res)
}

#[derive(Serialize)]
pub struct CreateResponseData<T> {
    data: T,
    total: u32,
    message: String,
    status_code: u16,
}

pub async fn create_response_data<T>(
    data: T,
    total: u32,
    message: String,
    status_code: u16,
) -> impl Responder
where
    T: Serialize,
{
    let res: CreateResponseData<T> = CreateResponseData {
        data,
        total,
        message,
        status_code,
    };

    HttpResponse::build(StatusCode::from_u16(status_code).unwrap()).json(res)
}

#[allow(dead_code)]
pub enum HttpStatus {
    Continue = 100,
    SwitchingProtocols = 101,
    Processing = 102,
    Earlyhints = 103,
    Ok = 200,
    Created = 201,
    Accepted = 202,
    NonAuthoritativeInformation = 203,
    NoContent = 204,
    ResetContent = 205,
    PartialContent = 206,
    Ambiguous = 300,
    MovedPermanently = 301,
    Found = 302,
    SeeOther = 303,
    NotModified = 304,
    TemporaryRedirect = 307,
    PermanentRedirect = 308,
    BadRequest = 400,
    UNAUTHORIZED = 401,
    PaymentRequired = 402,
    Forbidden = 403,
    NotFound = 404,
    MethodNotAllowed = 405,
    NotAcceptable = 406,
    ProxyAuthenticationRequired = 407,
    RequestTimeout = 408,
    Conflict = 409,
    Gone = 410,
    LengthRequired = 411,
    PreconditionFailed = 412,
    PayloadTooLarge = 413,
    UriTooLong = 414,
    UnsupportedMediaType = 415,
    RequestedRangeNotSatisfiable = 416,
    ExpectationFailed = 417,
    IAmATeapot = 418,
    MISDIRECTED = 421,
    UnprocessableEntity = 422,
    FailedDependency = 424,
    PreconditionRequired = 428,
    TooManyRequests = 429,
    InternalServerError = 500,
    NotImplemented = 501,
    BadGateway = 502,
    ServiceUnavailable = 503,
    GatewayTimeout = 504,
    HttpVersionNotSupported = 505
}