#pragma once

#include <expected>
#include <functional>
#include <optional>
#include <string>
#include <utility>
#include <variant>

namespace lexer {
struct TokenLocation {
    unsigned int line;
    unsigned int column;
};

enum class SimpleTokenType {};

struct SimpleToken {
    TokenLocation location;
    SimpleTokenType type;
};

enum class ComplexTokenType {};

struct ComplexToken {
    TokenLocation location;
    ComplexTokenType type;
    std::string lexeme;
};

using Token = std::variant<SimpleToken, ComplexToken>;
using TokenOrTokenPair = std::variant<Token, std::pair<Token, Token>>;

using LexerError = std::variant<>;

using SuccessResult = std::optional<TokenOrTokenPair>;
using SuccessResultOrError = std::expected<SuccessResult, LexerError>;
using LexerFeedFunc = std::function<SuccessResultOrError(char c) noexcept>;
};
