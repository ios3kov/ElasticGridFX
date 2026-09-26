#pragma once
#include "core/GridModel.h"
#include <cstdint>
#include <span>
#include <vector>

namespace elasticgrid {

// Versioned, endian-stable serialization for clean-room grid state. The
// current Rust AE host has its own serde-backed arbitrary-data payload; this
// codec remains the portable C++ format for tests/future cross-host exchange.
std::vector<std::uint8_t> encodeGridState(const GridState& state);

// Returns false for malformed/corrupt/unsupported blobs and leaves `out`
// untouched. Decoded guides are validated to remain strictly monotonic.
bool decodeGridState(std::span<const std::uint8_t> blob, GridState& out);

// Deterministic interpolation for keyframed arbitrary data. Topology changes
// are stepped because guide counts cannot be meaningfully interpolated.
GridState interpolateGridState(const GridState& a, const GridState& b, float t,
                               float min_spacing = 0.0f);

} // namespace elasticgrid
