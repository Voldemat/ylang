#include "src/parsers/file.hpp"

#include <ranges>
#include <variant>

#include "src/lexer.hpp"
#include "src/source_file.hpp"
#include "src/utils.hpp"

namespace parsers::file {

using PFTReturnType = std::expected<ast::Node, Error>;

PFTReturnType parseFromSimpleToken(const GetNextTokenFunc &getNextToken,
                                   const std::weak_ptr<SourceFile> &sourceFile,
                                   const lexer::SimpleToken &token) {
    return std::unexpected((UnexpectedToken)token);
};

std::expected<shared::ast::NameNode, Error> parseNameNode(
    const GetNextTokenFunc &getNextToken,
    const std::weak_ptr<SourceFile> &sourceFile) {
    std::vector<lexer::ComplexToken> tokens;
    bool lastTokenIsIdentifier = true;
    while (true) {
        const auto &t = getNextToken();
        if (!t.has_value()) {
            break;
        };
        const auto &currentToken = t.value();
        const auto& error = std::visit(overloaded{
            [&tokens](const lexer::ComplexToken& cToken) -> std::optional<Error>{
                if (cToken.type != lexer::ComplexTokenType::IDENTIFIER) {
                    return (UnexpectedToken)cToken;
                };
                tokens.emplace_back(cToken);
                return std::nullopt;
            },
            [](const lexer::SimpleToken& sToken) -> std::optional<Error> {
                return std::nullopt;
            }
        }, currentToken);
        if (error.has_value()) {
            return std::unexpected(error.value());
        };
    };
    if (tokens.size() == 0) {
        return std::unexpected((UnexpectedEOF){});
    };
    return (shared::ast::NameNode){
        .location = {
            .startToken = tokens.front(),
            .endToken = tokens.back(),
            .sourceFile = sourceFile,
        },
        .parts = tokens | std::views::transform([](const auto& token){
            return token.lexeme;
        }) | std::ranges::to<std::vector>(),
    };
};

PFTReturnType parseFunctionDefinition(
    const GetNextTokenFunc &getNextToken,
    const std::weak_ptr<SourceFile> &sourceFile) {
    const auto& nameResult = parseNameNode(getNextToken, sourceFile);
    if (!nameResult.has_value()) return std::unexpected(nameResult.error());
    const auto& nameNode = nameResult.value();
};

PFTReturnType parseFromComplexToken(const GetNextTokenFunc &getNextToken,
                                    const std::weak_ptr<SourceFile> &sourceFile,
                                    const lexer::ComplexToken &token) {
    if (token.type == lexer::ComplexTokenType::IDENTIFIER &&
        token.lexeme == "func") {
        return parseFunctionDefinition(getNextToken, sourceFile);
    };
    return std::unexpected((UnexpectedToken)token);
};

PFTReturnType parseFromToken(const GetNextTokenFunc &getNextToken,
                             const std::weak_ptr<SourceFile> &sourceFile,
                             const lexer::Token &token) {
    return std::visit(
        overloaded{
            [&getNextToken,
             &sourceFile](const lexer::SimpleToken &sToken) -> PFTReturnType {
                return parseFromSimpleToken(getNextToken, sourceFile, sToken);
            },
            [&getNextToken,
             &sourceFile](const lexer::ComplexToken &cToken) -> PFTReturnType {
                return parseFromComplexToken(getNextToken, sourceFile, cToken);
            } },
        token);
};

ReturnType parse(const GetNextTokenFunc &getNextToken,
                 const std::weak_ptr<SourceFile> &sourceFile) {
    Result nodes;
    Errors errors;
    std::optional<lexer::Token> currentToken;
    while (true) {
        currentToken = getNextToken();
        if (!currentToken.has_value()) {
            break;
        };
        const auto &node =
            parseFromToken(getNextToken, sourceFile, currentToken.value());
        if (node.has_value()) {
            nodes.emplace_back(node.value());
        } else {
            errors.emplace_back(node.error());
        };
    };
};
};  // namespace parsers::file
