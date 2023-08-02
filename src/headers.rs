use axum::headers::{Error, Header, HeaderName, HeaderValue};

/// The `HX-Request` header.
///
/// Per documentation, this is always `true` so we don't care about the value and just check for its presence.
pub struct HxRequest {
    _priv: (),
}

static INSTANCE: HeaderName = HeaderName::from_static("hx-request");

impl Header for HxRequest {
    fn name() -> &'static HeaderName {
        &INSTANCE
    }

    fn decode<'i, I>(values: &mut I) -> Result<Self, Error>
    where
        Self: Sized,
        I: Iterator<Item = &'i HeaderValue>,
    {
        values
            .next()
            .map(|_| HxRequest { _priv: () })
            .ok_or_else(Error::invalid)
    }

    fn encode<E: Extend<HeaderValue>>(&self, values: &mut E) {
        values.extend([HeaderValue::from_static("true")]);
    }
}
