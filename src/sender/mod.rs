// Copyright 2024 FastLabs Developers
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use std::fmt;
use std::io;

use crate::SDElement;
use crate::Severity;

#[cfg(unix)]
mod unix;
#[cfg(unix)]
pub use unix::*;

mod tcp;
pub use tcp::*;

mod udp;
pub use udp::*;

#[macro_export]
macro_rules! impl_syslog_sender {
    ($sender:ident, $context:ident, $writer:ident) => {
        impl $sender {
            /// Send a message with the given severity as defined in RFC-3164.
            pub fn send_rfc3164<M: std::fmt::Display>(
                &mut self,
                severity: $crate::Severity,
                message: M,
            ) -> std::io::Result<()> {
                use std::io::Write;
                let message = &self.$context.format_rfc3164(severity, Some(message));
                write!(&mut self.$writer, "{message}")
            }

            /// Send a message with the given severity as defined in RFC-5424.
            pub fn send_rfc5424<S: Into<String>, M: std::fmt::Display>(
                &mut self,
                severity: $crate::Severity,
                msgid: Option<S>,
                elements: Vec<$crate::SDElement>,
                message: M,
            ) -> std::io::Result<()> {
                use std::io::Write;
                let message =
                    &self
                        .$context
                        .format_rfc5424(severity, msgid, elements, Some(message));
                write!(&mut self.$writer, "{message}")
            }

            /// Send a message with the given severity as defined in RFC-5425.
            pub fn send_rfc5425<S: Into<String>, M: std::fmt::Display>(
                &mut self,
                severity: $crate::Severity,
                msgid: Option<S>,
                elements: Vec<$crate::SDElement>,
                message: M,
            ) -> std::io::Result<()> {
                use std::io::Write;
                let message =
                    &self
                        .$context
                        .format_rfc5425(severity, msgid, elements, Some(message));
                write!(&mut self.$writer, "{message}")
            }
        }
    };
}

#[derive(Debug)]
pub enum SyslogSender {
    Tcp(TcpSender),
    Udp(UdpSender),
    #[cfg(unix)]
    UnixDatagram(UnixDatagramSender),
    #[cfg(unix)]
    UnixStream(UnixStreamSender),
}

impl SyslogSender {
    /// Send a message with the given severity as defined in RFC-3164.
    pub fn send_rfc3164<M: fmt::Display>(
        &mut self,
        severity: Severity,
        message: M,
    ) -> io::Result<()> {
        match self {
            SyslogSender::Tcp(sender) => sender.send_rfc3164(severity, message),
            SyslogSender::Udp(sender) => sender.send_rfc3164(severity, message),
            #[cfg(unix)]
            SyslogSender::UnixDatagram(sender) => sender.send_rfc3164(severity, message),
            #[cfg(unix)]
            SyslogSender::UnixStream(sender) => sender.send_rfc3164(severity, message),
        }
    }

    /// Send a message with the given severity as defined in RFC-5424.
    pub fn send_rfc5424<S: Into<String>, M: fmt::Display>(
        &mut self,
        severity: Severity,
        msgid: Option<S>,
        elements: Vec<SDElement>,
        message: M,
    ) -> io::Result<()> {
        match self {
            SyslogSender::Tcp(sender) => sender.send_rfc5424(severity, msgid, elements, message),
            SyslogSender::Udp(sender) => sender.send_rfc5424(severity, msgid, elements, message),
            #[cfg(unix)]
            SyslogSender::UnixDatagram(sender) => {
                sender.send_rfc5424(severity, msgid, elements, message)
            }
            #[cfg(unix)]
            SyslogSender::UnixStream(sender) => {
                sender.send_rfc5424(severity, msgid, elements, message)
            }
        }
    }

    /// Send a message with the given severity as defined in RFC-5425.
    pub fn send_rfc5425<S: Into<String>, M: fmt::Display>(
        &mut self,
        severity: Severity,
        msgid: Option<S>,
        elements: Vec<SDElement>,
        message: M,
    ) -> io::Result<()> {
        match self {
            SyslogSender::Tcp(sender) => sender.send_rfc5425(severity, msgid, elements, message),
            SyslogSender::Udp(sender) => sender.send_rfc5425(severity, msgid, elements, message),
            #[cfg(unix)]
            SyslogSender::UnixDatagram(sender) => {
                sender.send_rfc5425(severity, msgid, elements, message)
            }
            #[cfg(unix)]
            SyslogSender::UnixStream(sender) => {
                sender.send_rfc5425(severity, msgid, elements, message)
            }
        }
    }

    /// Flush the underlying writer if needed.
    pub fn flush(&mut self) -> io::Result<()> {
        match self {
            SyslogSender::Tcp(sender) => sender.flush(),
            SyslogSender::UnixStream(sender) => sender.flush(),
            #[cfg(unix)]
            SyslogSender::Udp(_) => Ok(()),
            #[cfg(unix)]
            SyslogSender::UnixDatagram(_) => Ok(()),
        }
    }
}