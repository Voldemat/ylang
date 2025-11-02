#pragma once

#include <expected>
#include <functional>
#include <memory>
#include <optional>
#include <variant>

#include "../lexer.hpp"
#include "../source_file.hpp"

namespace parsers::module {
namespace ast {
using Node = std::variant<>;
};  // namespace ast

using Error = std::variant<>;
using Result = std::optional<ast::Node>;
using ReturnType = std::expected<Result, Error>;

using Func = std::function<ReturnType(
    const lexer::Token &token,
    const std::weak_ptr<SourceFile> &sourceFile) noexcept>;

};  // namespace parsers::module
