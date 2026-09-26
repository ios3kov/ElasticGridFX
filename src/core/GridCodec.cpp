#include "core/GridCodec.h"
#include <algorithm>
#include <bit>
#include <cmath>
#include <cstring>
#include <limits>

namespace elasticgrid {
namespace {
constexpr std::uint32_t kMagic = 0x58464745u; // 'EGFX' little-endian
constexpr std::uint16_t kVersion = 1;
constexpr std::size_t kMaxCells = 128;

void putU16(std::vector<std::uint8_t>& b, std::uint16_t v) {
    b.push_back(static_cast<std::uint8_t>(v));
    b.push_back(static_cast<std::uint8_t>(v >> 8));
}
void putU32(std::vector<std::uint8_t>& b, std::uint32_t v) {
    for (int i = 0; i < 4; ++i) b.push_back(static_cast<std::uint8_t>(v >> (i * 8)));
}
void putF32(std::vector<std::uint8_t>& b, float v) {
    putU32(b, std::bit_cast<std::uint32_t>(v));
}

bool getU16(std::span<const std::uint8_t> b, std::size_t& p, std::uint16_t& v) {
    if (p + 2 > b.size()) return false;
    v = static_cast<std::uint16_t>(b[p]) |
        static_cast<std::uint16_t>(static_cast<std::uint16_t>(b[p + 1]) << 8);
    p += 2;
    return true;
}
bool getU32(std::span<const std::uint8_t> b, std::size_t& p, std::uint32_t& v) {
    if (p + 4 > b.size()) return false;
    v = static_cast<std::uint32_t>(b[p]) |
        (static_cast<std::uint32_t>(b[p + 1]) << 8) |
        (static_cast<std::uint32_t>(b[p + 2]) << 16) |
        (static_cast<std::uint32_t>(b[p + 3]) << 24);
    p += 4;
    return true;
}
bool getF32(std::span<const std::uint8_t> b, std::size_t& p, float& v) {
    std::uint32_t bits = 0;
    if (!getU32(b, p, bits)) return false;
    v = std::bit_cast<float>(bits);
    return std::isfinite(v);
}

std::uint32_t fnv1a(std::span<const std::uint8_t> b) {
    std::uint32_t h = 2166136261u;
    for (auto x : b) { h ^= x; h *= 16777619u; }
    return h;
}

bool validLines(const std::vector<float>& v) {
    if (v.size() < 2) return false;
    if (std::abs(v.front()) > 1e-5f || std::abs(v.back() - 1.0f) > 1e-5f) return false;
    for (std::size_t i = 1; i < v.size(); ++i) {
        if (!(v[i] > v[i - 1]) || v[i] < 0.0f || v[i] > 1.0f) return false;
    }
    return true;
}

bool restoreAxis(const std::vector<float>& lines,
                 const std::vector<std::uint8_t>& pins,
                 AxisGrid& out) {
    if (!validLines(lines) || lines.size() != pins.size()) return false;
    AxisGrid tmp(lines.size() - 1);
    for (std::size_t i = 1; i + 1 < lines.size(); ++i) {
        if (!tmp.setLinePosition(i, lines[i], 0.0f)) return false;
    }
    for (std::size_t i = 1; i + 1 < pins.size(); ++i) {
        if (pins[i] && !tmp.setPinned(i, true)) return false;
    }
    out = std::move(tmp);
    return true;
}
} // namespace

std::vector<std::uint8_t> encodeGridState(const GridState& state) {
    const auto& cx = state.columns.lines();
    const auto& cy = state.rows.lines();
    const auto& px = state.columns.pins();
    const auto& py = state.rows.pins();

    std::vector<std::uint8_t> b;
    b.reserve(24 + (cx.size() + cy.size()) * 5);
    putU32(b, kMagic);
    putU16(b, kVersion);
    putU16(b, 0); // flags/reserved
    putU16(b, static_cast<std::uint16_t>(state.columns.cells()));
    putU16(b, static_cast<std::uint16_t>(state.rows.cells()));
    putU32(b, 0); // future payload flags
    for (float v : cx) putF32(b, v);
    for (float v : cy) putF32(b, v);
    b.insert(b.end(), px.begin(), px.end());
    b.insert(b.end(), py.begin(), py.end());
    putU32(b, fnv1a(b));
    return b;
}

bool decodeGridState(std::span<const std::uint8_t> blob, GridState& out) {
    if (blob.size() < 20) return false;
    const std::size_t checksum_pos = blob.size() - 4;
    std::size_t cp = checksum_pos;
    std::uint32_t stored = 0;
    if (!getU32(blob, cp, stored) || stored != fnv1a(blob.first(checksum_pos))) return false;

    std::size_t p = 0;
    std::uint32_t magic = 0, payload_flags = 0;
    std::uint16_t version = 0, reserved = 0, cols = 0, rows = 0;
    if (!getU32(blob, p, magic) || !getU16(blob, p, version) || !getU16(blob, p, reserved) ||
        !getU16(blob, p, cols) || !getU16(blob, p, rows) || !getU32(blob, p, payload_flags)) return false;
    if (magic != kMagic || version != kVersion || reserved != 0 || payload_flags != 0) return false;
    if (cols < 1 || rows < 1 || cols > kMaxCells || rows > kMaxCells) return false;

    const std::size_t x_count = static_cast<std::size_t>(cols) + 1;
    const std::size_t y_count = static_cast<std::size_t>(rows) + 1;
    const std::size_t expected = 16 + (x_count + y_count) * 4 + x_count + y_count + 4;
    if (blob.size() != expected) return false;

    std::vector<float> lx(x_count), ly(y_count);
    for (float& v : lx) if (!getF32(blob, p, v)) return false;
    for (float& v : ly) if (!getF32(blob, p, v)) return false;
    std::vector<std::uint8_t> px(x_count), py(y_count);
    for (auto& v : px) { if (p >= checksum_pos) return false; v = blob[p++]; if (v > 1) return false; }
    for (auto& v : py) { if (p >= checksum_pos) return false; v = blob[p++]; if (v > 1) return false; }
    if (p != checksum_pos || px.front() != 1 || px.back() != 1 || py.front() != 1 || py.back() != 1) return false;

    GridState tmp{AxisGrid(cols), AxisGrid(rows)};
    if (!restoreAxis(lx, px, tmp.columns) || !restoreAxis(ly, py, tmp.rows)) return false;
    out = std::move(tmp);
    return true;
}

GridState interpolateGridState(const GridState& a, const GridState& b, float t, float min_spacing) {
    t = std::clamp(t, 0.0f, 1.0f);
    if (a.columns.cells() != b.columns.cells() || a.rows.cells() != b.rows.cells())
        return t < 0.5f ? a : b;

    GridState out{AxisGrid(a.columns.cells()), AxisGrid(a.rows.cells())};
    auto blendAxis = [t, min_spacing](const AxisGrid& aa, const AxisGrid& bb, AxisGrid& dst) {
        const auto& la = aa.lines();
        const auto& lb = bb.lines();
        std::vector<float> blended(la.size());
        for (std::size_t i = 0; i < la.size(); ++i) blended[i] = std::lerp(la[i], lb[i], t);
        AxisGrid::enforceMonotonic(blended, min_spacing, nullptr);
        for (std::size_t i = 1; i + 1 < blended.size(); ++i) dst.setLinePosition(i, blended[i], min_spacing);
        const auto& chosenPins = (t < 0.5f) ? aa.pins() : bb.pins();
        for (std::size_t i = 1; i + 1 < chosenPins.size(); ++i) if (chosenPins[i]) dst.setPinned(i, true);
    };
    blendAxis(a.columns, b.columns, out.columns);
    blendAxis(a.rows, b.rows, out.rows);
    return out;
}

} // namespace elasticgrid
