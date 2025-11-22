#include "./lexer.hpp"

#include <cassert>
#include <cctype>

#include "src/lexer.hpp"

namespace implementation::lexer {
::lexer::ComplexToken extractComplexToken(::lexer::State &state) {
    auto currentToken = state.currentComplexToken.value();
    state.currentComplexToken = std::nullopt;
    return currentToken;
};

std::optional<::lexer::ComplexToken> extractComplexTokenIfPresent(
    ::lexer::State &state) {
    return state.currentComplexToken.has_value()
               ? std::make_optional(extractComplexToken(state))
               : std::nullopt;
};

const auto &numberCondition =
    std::function([](const char &c, const std::string &buffer) {
        return std::isdigit(c) != 0 || !buffer.contains('.') && c == '.';
    });
const auto &stringCondition =
    std::function([](const char &c, const std::string &buffer) {
        char quote = buffer.front();
        char lastChar = buffer.back();
        char preLastChar = buffer[buffer.length() - 1];
        if (lastChar == quote && preLastChar != '\\' && buffer.size() != 1)
            return false;
        return true;
    });
const auto &identifierCondition =
    std::function([](const char &c, const std::string &buffer) {
        return std::isalpha(c) || std::isdigit(c) || c == '_';
    });

using Condition = std::function<bool(char, std::string)>;

::lexer::ReturnType feedNewChar(::lexer::State &state, char c);

std::optional<::lexer::SimpleTokenType> simpleTokenTypeFromChar(char c) {
    switch (c) {
        case ',':
            return ::lexer::SimpleTokenType::COMMA;
        case ';':
            return ::lexer::SimpleTokenType::SEMICOLON;
        case ':':
            return ::lexer::SimpleTokenType::COLON;
        case '(':
            return ::lexer::SimpleTokenType::LEFT_PAREN;
        case ')':
            return ::lexer::SimpleTokenType::RIGHT_PAREN;
        case '{':
            return ::lexer::SimpleTokenType::LEFT_BRACE;
        case '}':
            return ::lexer::SimpleTokenType::RIGHT_BRACE;
        case '[':
            return ::lexer::SimpleTokenType::LEFT_BRACKET;
        case ']':
            return ::lexer::SimpleTokenType::RIGHT_BRACKET;
        case '@':
            return ::lexer::SimpleTokenType::AT_SIGN;
        case '=':
            return ::lexer::SimpleTokenType::EQUAL;
        case '>':
            return ::lexer::SimpleTokenType::GREATER_THAN;
        case '<':
            return ::lexer::SimpleTokenType::LESS_THAN;
        case '?':
            return ::lexer::SimpleTokenType::QUESTION_MARK;
        case '!':
            return ::lexer::SimpleTokenType::EXCLAMATION_MARK;
        case '&':
            return ::lexer::SimpleTokenType::AMPERSAND;
        case '/':
            return ::lexer::SimpleTokenType::SLASH;
        case '-':
            return ::lexer::SimpleTokenType::DASH;
        case '|':
            return ::lexer::SimpleTokenType::SLASH;
        case '+':
            return ::lexer::SimpleTokenType::PLUS;
        case '.':
            return ::lexer::SimpleTokenType::POINT;
        default:
            return std::nullopt;
    };
};

std::optional<::lexer::ComplexTokenType> complexTokenTypeFromChar(char c) {
    if (c == '"' || c == '\'') return ::lexer::ComplexTokenType::STRING_LITERAL;
    if (std::isdigit(c) != 0) return ::lexer::ComplexTokenType::NUMBER_LITERAL;
    if (std::isalpha(c) != 0) return ::lexer::ComplexTokenType::IDENTIFIER;
    return std::nullopt;
};

std::optional<::lexer::SimpleTokenType> getMergedSimpleTokenType(
    const ::lexer::SimpleToken &lastToken,
    const ::lexer::SimpleTokenType &currentTokenType) {
    if (lastToken.type == ::lexer::SimpleTokenType::COLON &&
        currentTokenType == ::lexer::SimpleTokenType::COLON) {
        return ::lexer::SimpleTokenType::DOUBLE_COLON;
    };
    return std::nullopt;
};

bool shouldWaitForPossibleDouble(const ::lexer::SimpleTokenType &type) {
    return type == ::lexer::SimpleTokenType::COLON;
};

::lexer::ReturnType feedWithoutComplexToken(
    ::lexer::State &state, char c) {
    state.location.column++;
    if (state.skipUntilNextLine || c == ' ' || c == '\r' || c == '\t')
        return {std::nullopt, false};
    if (c == '#') {
        state.skipUntilNextLine = true;
        return {std::nullopt, false};
    };
    const auto &simpleTokenTypeOptional = simpleTokenTypeFromChar(c);
    if (simpleTokenTypeOptional.has_value()) {
        const auto &simpleTokenType = simpleTokenTypeOptional.value();
        const auto &newToken =
            state.lastSimpleToken
                .and_then([&simpleTokenType](const auto &lastSimpleToken) {
                    return getMergedSimpleTokenType(lastSimpleToken,
                                                    simpleTokenType)
                        .transform(
                            [&lastSimpleToken](const auto &newTokenType) {
                                return std::make_pair(lastSimpleToken,
                                                      newTokenType);
                            });
                })
                .transform([&state](const auto &pair) -> ::lexer::SimpleToken {
                    const auto &[lastSimpleToken, newTokenType] = pair;
                    state.lastSimpleToken = std::nullopt;
                    return { .location = lastSimpleToken.location,
                             .type = newTokenType };
                })
                .or_else([&state, &simpleTokenType]()
                             -> std::optional<::lexer::SimpleToken> {
                    const auto& token = (::lexer::SimpleToken){
                        .location = state.location,
                        .type = simpleTokenType,
                    };
                    if (shouldWaitForPossibleDouble(simpleTokenType)) {
                        state.lastSimpleToken = token;
                        return std::nullopt;
                    };
                    return token;
                });
        return {newToken, false};
    } else {
        if (state.lastSimpleToken.has_value()) {
            const auto& simpleToken = state.lastSimpleToken.value();
            state.lastSimpleToken = std::nullopt;
            return {simpleToken, true};
        };
        const auto &complexTokenTypeOptional = complexTokenTypeFromChar(c);
        if (complexTokenTypeOptional.has_value()) {
            state.currentComplexToken = (::lexer::ComplexToken){
                .location = state.location,
                .type = complexTokenTypeOptional.value(),
                .lexeme = std::string(1, c),
            };
            return {std::nullopt, false};
        };
        return {std::unexpected((::lexer::UnexpectedChar){
            .location = state.location,
            .c = c,
        }), false};
    };
};

::lexer::ReturnType feedWithCondition(::lexer::State &state,
                                      ::lexer::ComplexToken &token,
                                      const Condition &condition, char c) {
    if (condition(c, token.lexeme)) {
        token.lexeme += c;
        state.location.column++;
        return {std::nullopt, false};
    };
    const auto &firstToken = extractComplexToken(state);
    return {firstToken, true};
};

Condition conditionFromComplexTokenType(
    const ::lexer::ComplexTokenType &tokenType) {
    switch (tokenType) {
        case ::lexer::ComplexTokenType::IDENTIFIER:
            return identifierCondition;
        case ::lexer::ComplexTokenType::STRING_LITERAL:
            return stringCondition;
        case ::lexer::ComplexTokenType::NUMBER_LITERAL:
            return numberCondition;
    };
};

::lexer::ReturnType feedWithComplexToken(::lexer::State &state, char c) {
    assert(state.currentComplexToken.has_value());
    auto &complexToken = state.currentComplexToken.value();
    const auto &condition = conditionFromComplexTokenType(complexToken.type);
    return feedWithCondition(state, complexToken, condition, c);
};

::lexer::ReturnType handleNewLine(::lexer::State &state, char c) {
    const auto &result = extractComplexTokenIfPresent(state);
    state.location.line++;
    state.location.column = 0;
    state.skipUntilNextLine = false;
    return {result, false};
};

::lexer::ReturnType feedNewChar(::lexer::State &state, char c) {
    return state.currentComplexToken.has_value()
               ? feedWithComplexToken(state, c)
               : feedWithoutComplexToken(state, c);
};

::lexer::ReturnType lexerFunc(::lexer::State &state, char c) {
    return c == '\n' ? handleNewLine(state, c) : feedNewChar(state, c);
};
};  // namespace implementation::lexer
