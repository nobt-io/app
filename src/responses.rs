use axum::body::{Bytes, Full};
use axum::http::{header, HeaderValue};
use axum::response::{IntoResponse, Response};

#[derive(Clone, Copy, Debug)]
pub struct Jpeg<T>(pub T);

impl<T> IntoResponse for Jpeg<T>
where
    T: Into<Full<Bytes>>,
{
    fn into_response(self) -> Response {
        (
            [(
                header::CONTENT_TYPE,
                HeaderValue::from_static(mime::IMAGE_JPEG.as_ref()),
            )],
            self.0.into(),
        )
            .into_response()
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Css<T>(pub T);

impl<T> IntoResponse for Css<T>
where
    T: Into<Full<Bytes>>,
{
    fn into_response(self) -> Response {
        (
            [(
                header::CONTENT_TYPE,
                HeaderValue::from_static(mime::TEXT_CSS.as_ref()),
            )],
            self.0.into(),
        )
            .into_response()
    }
}
