#include <fstream>
#include <print>
#include <sstream>
#include <string>
#include "src/utils.hpp"

#include "src/implementation/lexer/lexer.hpp"
#include "src/lexer.hpp"

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

    std::istringstream stream(content);
    lexer::State state;
    std::vector<lexer::Token> tokens;
    std::vector<lexer::Error> errors;
    while (true) {
        char c = stream.get();
        if (stream.eof()) break;
        bool feedAgain = true;
        while (feedAgain) {
            const auto &result =
                implementation::lexer::lexerFunc(state, c);
            feedAgain = result.second;
            if (!result.first.has_value()) continue;
            appendTokenOrError(tokens, errors, result.first.value());
        };
    };
    if (state.currentComplexToken.has_value()) {
        tokens.emplace_back(state.currentComplexToken.value());
    };
    if (state.lastSimpleToken.has_value()) {
        tokens.emplace_back(state.lastSimpleToken.value());
    };
    lexer::Func func = implementation::lexer::lexerFunc;
    unsigned int n = 1;
    for (const auto &token : tokens) {
        std::println("{}: {}", n, token);
        n++;
    };
    for (const auto &error : errors) {
        std::println("{}", error);
    };
};
