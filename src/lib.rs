//! Zero2prod - Une application web Rust avec instrumentation OpenTelemetry.
//!
//! Ce crate fournit une structure modulaire avec :
//! - `app` : Configuration et métadonnées de l'application
//! - `otel` : Instrumentation et télémétrie OpenTelemetry
//! - `webapp` : Serveur HTTP Axum avec middleware

#![warn(clippy::all)]
#![allow(dead_code)]

pub mod app;
pub mod otel;
pub mod server;
