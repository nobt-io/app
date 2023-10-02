use axum::body::{Bytes, Full};
use axum::http::{header, HeaderValue};
use axum::response::{IntoResponse, Response};

macro_rules! mime_response {
    ($name:tt, $mime:expr) => {
        #[derive(Clone, Copy, Debug)]
        pub struct $name<T>(pub T);

        impl<T> IntoResponse for $name<T>
        where
            T: Into<Full<Bytes>>,
        {
            fn into_response(self) -> Response {
                (
                    [(
                        header::CONTENT_TYPE,
                        HeaderValue::from_static($mime.as_ref()),
                    )],
                    self.0.into(),
                )
                    .into_response()
            }
        }
    };
}

mime_response!(Jpeg, mime::IMAGE_JPEG);
mime_response!(Css, mime::TEXT_CSS);
mime_response!(Javascript, mime::APPLICATION_JAVASCRIPT);
mime_response!(Png, mime::IMAGE_PNG);
