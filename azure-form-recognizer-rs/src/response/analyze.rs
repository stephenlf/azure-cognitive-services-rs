pub struct ErrorBody {
    pub error: Box<TopLevelError>
}

pub struct TopLevelError {
    pub code: ErrorCode,
    pub message: String,
    pub innererror: Box<InnerError>,
    pub details: Vec::<InnerError>,
}

pub struct InnerError {
    pub code: InnerErrorCode,
    pub message: String,
    pub target: String,
}

pub enum ErrorCode {
    InvalidRequest,
    InvalidArgument,
    Forbidden,
    UnsupportedMediaType,
    InternalServerError,
    ServiceUnavailable,
}

pub enum InnerErrorCode {
    InvalidContentSourceFormat,
    InvalidParameter,
    InvalidParameterLength,
    InvalidSasToken,
    ParameterMissing,
    InvalidContent,
    InvalidContentDimensions,
    InvalidContentLength,
    NotSupportedApiVersion,
    AuthorizationFailed,
    InvalidDataProtectionKey,
    OutboundAccessForbidden,
    UnsupportedMediaType,
    Unknow,
    ServiceUnavailable,

}