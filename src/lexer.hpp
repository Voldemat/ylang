#pragma once

#include <expected>
#include <format>
#include <functional>
#include <optional>
#include <string>
#include <utility>
#include <variant>

#include "magic_enum.hpp"

namespace lexer {
struct TokenLocation {
    unsigned int line = 1;
    unsigned int column = 1;
};

enum class SimpleTokenType {
    LEFT_BRACKET,
    RIGHT_BRACKET,
    LEFT_PAREN,
    RIGHT_PAREN,
    LEFT_BRACE,
    RIGHT_BRACE,
    AT_SIGN,
    COMMA,
    COLON,
    SEMICOLON,
    EQUAL,
    GREATER_THAN,
    LESS_THAN,
    QUESTION_MARK,
    EXCLAMATION_MARK,
    AMPERSAND,
    SLASH,
    DASH,
    VSLASH,
    PLUS,
    POINT
};

struct SimpleToken {
    TokenLocation location;
    SimpleTokenType type;
};

enum class ComplexTokenType {
    IDENTIFIER,
    STRING_LITERAL,
    NUMBER_LITERAL,
    LINE_COMMENT,
};

struct ComplexToken {
    TokenLocation location;
    ComplexTokenType type;
    std::string lexeme;
};

using Token = std::variant<SimpleToken, ComplexToken>;

struct UnexpectedChar {
    TokenLocation location;
    char c;
};

using Error = std::variant<UnexpectedChar>;
using TokenOrError = std::expected<Token, Error>;
using TOEPair = std::pair<TokenOrError, TokenOrError>;
using TOEOrPair = std::variant<TokenOrError, TOEPair>;
using ReturnType = std::optional<TOEOrPair>;
struct State {
    lexer::TokenLocation location;
    std::optional<lexer::ComplexToken> currentComplexToken;
};
using Func = std::function<ReturnType(State &state, char c)>;

lexer::ReturnType lexerFunc(lexer::State &state, char c);
};  // namespace lexer

namespace std {
template <>
struct formatter<lexer::UnexpectedChar> {
    template <typename ParseContext>
    constexpr auto parse(ParseContext &ctx) {
        return ctx.begin();
    }

    template <typename FormatContext>
    auto format(const lexer::UnexpectedChar &record, FormatContext &ctx) const {
        return std::format_to(ctx.out(),
                              "UnexpectedChar(location: {}, c: \"{}\")",
                              record.location, record.c);
    }
};

template <>
struct formatter<lexer::SimpleToken> {
    template <typename ParseContext>
    constexpr auto parse(ParseContext &ctx) {
        return ctx.begin();
    }

    template <typename FormatContext>
    auto format(const lexer::SimpleToken &record, FormatContext &ctx) const {
        return std::format_to(ctx.out(), "SimpleToken(location: {}, type: {})",
                              record.location,
                              magic_enum::enum_name(record.type));
    }
};

template <>
struct formatter<lexer::ComplexToken> {
    template <typename ParseContext>
    constexpr auto parse(ParseContext &ctx) {
        return ctx.begin();
    }

    template <typename FormatContext>
    auto format(const lexer::ComplexToken &record, FormatContext &ctx) const {
        return std::format_to(
            ctx.out(), "ComplexToken(location: {}, type: {}, lexeme: \"{}\")",
            record.location, magic_enum::enum_name(record.type), record.lexeme);
    }
};

template <>
struct formatter<lexer::TokenLocation> {
    template <typename ParseContext>
    constexpr auto parse(ParseContext &ctx) {
        return ctx.begin();
    }

    template <typename FormatContext>
    auto format(const lexer::TokenLocation &record, FormatContext &ctx) const {
        return std::format_to(ctx.out(), "TokenLocation(line: {}, column: {})",
                              record.line, record.column);
    }
};
};  // namespace std
