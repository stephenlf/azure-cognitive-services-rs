pub struct ErrorBody {
    pub error: Option<Box<TopLevelError>>
}

pub struct TopLevelError {
    pub code: Option<ErrorCode>,
    pub message: Option<String>,
    pub innererror: Option<Box<InnerError>>,
    pub details: Option<Vec::<InnerError>>,
}

pub struct InnerError {
    pub code: Option<InnerErrorCode>,
    pub message: Option<String>,
    pub target: Option<String>,
}

pub enum ErrorCode {
    InvalidRequest,
    InvalidArgument,
    Forbidden,
    UnsupportedMediaType,
    InternalServerError,
    ServiceUnavailable,
    NotFound,
    Conflict,
    MethodNotAllowed,
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
    OperationNotFound,
    ContentSourceNotAccessible,
    ContentSourceTimeout,
    DocumentModelLimit,
    DocumentModelLimitComposed,
    InvalidFieldsDefinition,
    InvalidTrainingContentLength,
    InvalidTrainingContentPageCount,
    ModelAnalyzeError,
    ModelBuildError,
    ModelComposeError,
    ModelNotReady,
    ModelReadOnly,
    OperationNotCancellable,
    TrainingContentMissing,
    ModelExists,
    GeneralInvalidParameter,
    ModelNotFound,
    Unknown,
    DocumentModelLimitNeural,
    UnsupportedContent,
}