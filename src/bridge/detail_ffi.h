#pragma once
#include <cstdint>
#include <span>
#include "core/DetailMap.h"

// New opt-in entry points use this record. The old render/plane ABIs stay frozen.
struct EgDetailMaps {
    const float* columns;
    std::int32_t column_count;
    const float* rows;
    std::int32_t row_count;
};
static_assert(sizeof(EgDetailMaps) == 32);
inline bool eg_detail_valid(const EgDetailMaps* maps) noexcept {
    if (!maps) return true;
    auto valid = [](const float* values, std::int32_t count) {
        if (count == 0) return true;
        if (!values || count != static_cast<std::int32_t>(elasticgrid::kDetailSamples)) return false;
        return elasticgrid::validDetail({values, static_cast<std::size_t>(count)});
    };
    return valid(maps->columns, maps->column_count) && valid(maps->rows, maps->row_count);
}
inline std::span<const float> eg_detail_x(const EgDetailMaps* maps) noexcept {
    return maps && maps->column_count > 0 ? std::span<const float>{maps->columns, static_cast<std::size_t>(maps->column_count)} : std::span<const float>{};
}
inline std::span<const float> eg_detail_y(const EgDetailMaps* maps) noexcept {
    return maps && maps->row_count > 0 ? std::span<const float>{maps->rows, static_cast<std::size_t>(maps->row_count)} : std::span<const float>{};
}
