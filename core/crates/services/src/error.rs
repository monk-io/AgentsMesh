use agentsmesh_types::ServiceError;

/// Convert any error with an `Into<ServiceError>` impl to the FFI wire format.
/// Use this in `.map_err(wire)` instead of the lossy `.map_err(|e| e.to_string())`.
pub fn wire<E: Into<ServiceError>>(e: E) -> String {
    e.into().to_wire()
}
