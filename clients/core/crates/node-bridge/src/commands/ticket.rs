// Ticket NAPI commands — currently dead, kept as an empty module to
// preserve `mod ticket;` in commands/mod.rs. State mutation now flows
// through proto-state on the wasm/wasm-bindgen side and a planned NAPI
// counterpart in a follow-up PR. Read accessors / Connect-RPC pass-throughs
// belong in their respective layers (renderer → Connect-RPC → backend).
