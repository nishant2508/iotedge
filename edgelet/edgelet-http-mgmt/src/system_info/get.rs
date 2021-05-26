// Copyright (c) Microsoft. All rights reserved.

pub(crate) struct Route {}

#[async_trait::async_trait]
impl http_common::server::Route for Route {
    type ApiVersion = edgelet_http::ApiVersion;
    fn api_version() -> &'static dyn http_common::DynRangeBounds<Self::ApiVersion> {
        &((edgelet_http::ApiVersion::V2018_06_28)..)
    }

    type Service = crate::Service;
    fn from_uri(
        service: &Self::Service,
        path: &str,
        _query: &[(std::borrow::Cow<'_, str>, std::borrow::Cow<'_, str>)],
        _extensions: &http::Extensions,
    ) -> Option<Self> {
        if path != "/systeminfo" {
            return None;
        }

        Some(Route {})
    }

    type DeleteBody = serde::de::IgnoredAny;
    type DeleteResponse = ();

    type GetResponse = edgelet_core::module::SystemInfo;
    async fn get(self) -> http_common::server::RouteResponse<Self::GetResponse> {
        Ok((
            http::StatusCode::OK,
            edgelet_core::module::SystemInfo::default(),
        ))
    }

    type PostBody = serde::de::IgnoredAny;
    type PostResponse = ();

    type PutBody = serde::de::IgnoredAny;
    type PutResponse = ();
}
