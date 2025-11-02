#include <fstream>
#include <print>

#include "src/lexer.hpp"
#include "src/utils.hpp"

void appendTokenOrError(std::vector<lexer::Token> &tokens,
                        std::vector<lexer::Error> &errors,
                        const lexer::TokenOrError &toe) {
    if (toe.has_value()) {
        tokens.push_back(toe.value());
    } else {
        errors.push_back(toe.error());
    };
};

int main() {
    std::ifstream file("check.yl");
    std::string content((std::istreambuf_iterator<char>(file)),
                        std::istreambuf_iterator<char>());

    lexer::State state;
    std::vector<lexer::Token> tokens;
    std::vector<lexer::Error> errors;
    for (const auto &c : content) {
        std::println("main c: {}", c);
        const auto &result = lexer::lexerFunc(state, c);
        if (!result.has_value()) continue;
        std::visit(
            overloaded{
                [&tokens, &errors](const lexer::TokenOrError &toe) -> void {
                    appendTokenOrError(tokens, errors, toe);
                },
                [&tokens, &errors](
                    const std::pair<lexer::TokenOrError, lexer::TokenOrError>
                        &pair) -> void {
                    appendTokenOrError(tokens, errors, pair.first);
                    appendTokenOrError(tokens, errors, pair.second);
                } },
            result.value());
    };
    if (state.currentComplexToken.has_value()) {
        tokens.push_back(state.currentComplexToken.value());
    };
    for (const auto& token : tokens) {
        std::println("{}\n", token);
    };
    for (const auto& error : errors) {
        std::println("{}\n", error);
    };
};
