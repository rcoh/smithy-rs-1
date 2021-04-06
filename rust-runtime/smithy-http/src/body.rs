/*
 * Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
 * SPDX-License-Identifier: Apache-2.0.
 */

use bytes::Bytes;
use http::{HeaderMap, HeaderValue};
use http_body::Body;
use pin_project::pin_project;
use std::error::Error as StdError;
use std::fmt;
use std::fmt::{Debug, Formatter};
use std::pin::Pin;
use std::task::{Context, Poll};

pub type Error = Box<dyn StdError + Send + Sync>;

/// SdkBody type
///
/// This is the Body used for dispatching all HTTP Requests.
/// For handling responses, the type of the body will be controlled
/// by the HTTP stack.
///
/// TODO: Consider renaming to simply `Body`, although I'm concerned about naming headaches
/// between hyper::Body and our Body
/// TODO: Once we add streaming bodies, we will need a custom debug implementation
#[pin_project]
#[derive(Debug)]
pub struct SdkBody(#[pin] Inner);

type BoxBody = http_body::combinators::BoxBody<Bytes, Box<dyn StdError + Send + Sync>>;

#[pin_project(project = InnerProj)]
enum Inner {
    Once(#[pin] Option<Bytes>),
    Streaming(#[pin] hyper::Body),
    Dyn(#[pin] BoxBody),
    // Taken,
}

impl Debug for Inner {
    fn fmt(&self, _f: &mut Formatter<'_>) -> fmt::Result {
        todo!()
    }
}

impl SdkBody {
    pub fn from_hyper(body: hyper::Body) -> Self {
        Self(Inner::Streaming(body))
    }

    pub fn from_dyn(body: BoxBody) -> Self {
        Self(Inner::Dyn(body))
    }

    pub fn taken() -> Self {
        // TODO: extra variant
        SdkBody(Inner::Once(None))
    }

    fn poll_inner(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
    ) -> Poll<Option<Result<Bytes, Error>>> {
        match self.project().0.project() {
            InnerProj::Once(ref mut opt) => {
                let data = opt.take();
                match data {
                    Some(bytes) => Poll::Ready(Some(Ok(bytes))),
                    None => Poll::Ready(None),
                }
            }
            InnerProj::Streaming(body) => body.poll_data(cx).map_err(|e| e.into()),
            InnerProj::Dyn(box_body) => box_body.poll_data(cx),
        }
    }

    /// If possible, return a reference to this body as `&[u8]`
    ///
    /// If this SdkBody is NOT streaming, this will return the byte slab
    /// If this SdkBody is streaming, this will return `None`
    pub fn bytes(&self) -> Option<&[u8]> {
        match &self.0 {
            Inner::Once(Some(b)) => Some(&b),
            Inner::Once(None) => Some(&[]),
            // In the future, streaming variants will return `None`
            _ => None,
        }
    }

    pub fn try_clone(&self) -> Option<Self> {
        match &self.0 {
            Inner::Once(bytes) => Some(SdkBody(Inner::Once(bytes.clone()))),
            _ => None,
        }
    }
}

impl From<&str> for SdkBody {
    fn from(s: &str) -> Self {
        SdkBody(Inner::Once(Some(Bytes::copy_from_slice(s.as_bytes()))))
    }
}

impl From<Bytes> for SdkBody {
    fn from(bytes: Bytes) -> Self {
        SdkBody(Inner::Once(Some(bytes)))
    }
}

impl From<Vec<u8>> for SdkBody {
    fn from(data: Vec<u8>) -> Self {
        Self::from(Bytes::from(data))
    }
}

impl http_body::Body for SdkBody {
    type Data = Bytes;
    type Error = Error;

    fn poll_data(
        self: Pin<&mut Self>,
        _cx: &mut Context<'_>,
    ) -> Poll<Option<Result<Self::Data, Self::Error>>> {
        self.poll_inner(_cx)
    }

    fn poll_trailers(
        self: Pin<&mut Self>,
        _cx: &mut Context<'_>,
    ) -> Poll<Result<Option<HeaderMap<HeaderValue>>, Self::Error>> {
        Poll::Ready(Ok(None))
    }
}
