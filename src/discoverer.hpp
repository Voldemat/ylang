#pragma once

#include <expected>
#include <functional>
#include <variant>

#include "src/source_file.hpp"

namespace discoverer {
using Error = std::variant<>;
struct Result {
    SourceFile sourceFile;
    bool hasMore;
};
using ReturnType = std::expected<Result, Error>;

using Func = std::function<ReturnType() noexcept>;
};  // namespace discoverer
