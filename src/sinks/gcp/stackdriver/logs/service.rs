//! Service implementation for the `gcp_stackdriver_logs` sink.

use bytes::Bytes;
use http::{header::CONTENT_TYPE, Request, Uri};

use crate::{
    gcp::GcpAuthenticator,
    event::EventRef,
    sinks::{
        util::http::{HttpRequest, HttpServiceRequestBuilder},
        HTTPRequestBuilderSnafu,
    },
};
use snafu::ResultExt;

#[derive(Debug, Clone)]
pub(super) struct StackdriverLogsServiceRequestBuilder {
    pub(super) uri: Uri,
    pub(super) auth: GcpAuthenticator,
}

impl HttpServiceRequestBuilder<()> for StackdriverLogsServiceRequestBuilder {
    fn build(&self, mut request: HttpRequest<()>, _log: Option<EventRef<'_>>) -> Result<Request<Bytes>, crate::Error> {
        let builder = Request::post(self.uri.clone()).header(CONTENT_TYPE, "application/json");

        let mut request = builder
            .body(request.take_payload())
            .context(HTTPRequestBuilderSnafu)
            .map_err(Into::<crate::Error>::into)?;

        self.auth.apply(&mut request);

        Ok(request)
    }
}
