#pragma once

#include <expected>
#include <functional>
#include <memory>
#include <span>
#include <variant>
#include <vector>

#include "../lexer.hpp"
#include "../source_file.hpp"

namespace parsers::file {
namespace ast {
struct NodeLocation {
    lexer::Token startToken;
    lexer::Token endToken;
    std::weak_ptr<SourceFile> sourceFile;
};

using Node = std::variant<>;
};

using ParseError = std::variant<>;
using ParseResult = std::vector<ast::Node>;
using ParseResultOrError = std::expected<ParseResult, ParseError>;

using ParseFunction = std::function<ParseResultOrError(
    const std::span<lexer::Token> &tokens,
    const std::weak_ptr<SourceFile> &sourceFile) noexcept>;

};  // namespace parser
