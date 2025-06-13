#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error(transparent)]
    IOError(#[from] std::io::Error),

    #[error(transparent)]
    ParseIntError(#[from] std::num::ParseIntError),

    #[error(transparent)]
    NetworkParseError(#[from] cidr::errors::NetworkParseError),

    #[error(transparent)]
    AddressParseError(#[from] std::net::AddrParseError),

    #[cfg(target_family = "unix")]
    #[error(transparent)]
    NixError(#[from] nix::Error),

    #[error(transparent)]
    SerdeYamlError(#[from] serde_yaml::Error),

    #[error(transparent)]
    RequestError(#[from] rquest::Error),

    #[error(transparent)]
    LogParseError(#[from] tracing_subscriber::filter::ParseError),

    #[error(transparent)]
    LogSetGlobalDefaultError(#[from] tracing::subscriber::SetGlobalDefaultError),

    #[error(transparent)]
    ResolveError(#[from] hickory_resolver::error::ResolveError),

    #[error(transparent)]
    JsonExtractorRejection(#[from] axum::extract::rejection::JsonRejection),

    #[error(transparent)]
    AxumHttpError(#[from] axum::http::Error),

    #[error("Missing or invalid 'x-vqd-4' header")]
    MissingHeader,

    #[error("{0}")]
    BadRequest(String),

    #[error("You didn't provide an API key. You need to provide your API key in an Authorization header using Bearer auth (i.e. Authorization: Bearer YOUR_KEY), or as the password field (with blank username) if you're accessing the API from your browser and are prompted for a username and password. You can obtain an API key from https://platform.openai.com/account/api-keys.")]
    InvalidApiKey,

    #[error(transparent)]
    BoringSSLConfigError(#[from] rquest::boring::ssl::Error),
}
