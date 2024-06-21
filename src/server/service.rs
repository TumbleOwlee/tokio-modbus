// SPDX-FileCopyrightText: Copyright (c) 2017-2024 slowtec GmbH <post@slowtec.de>
// SPDX-License-Identifier: MIT OR Apache-2.0

use crate::Slave;
use std::{future::Future, ops::Deref};

/// A Modbus server service.
pub trait Service {
    /// Requests handled by the service.
    type Request;

    /// The future response value.
    type Future: Future<Output = Result<crate::Response, crate::Exception>> + Send;

    /// Process the request and return the response asynchronously.
    fn call(&self, slave: Slave, req: Self::Request) -> Self::Future;
}

impl<D> Service for D
where
    D: Deref + ?Sized,
    D::Target: Service,
{
    type Request = <D::Target as Service>::Request;
    type Future = <D::Target as Service>::Future;

    /// A forwarding blanket impl to support smart pointers around [`Service`].
    fn call(&self, slave: Slave, req: Self::Request) -> Self::Future {
        self.deref().call(slave, req)
    }
}
