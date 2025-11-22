#pragma once

#include <expected>
#include <functional>
#include <map>
#include <memory>
#include <optional>
#include <string>
#include <variant>
#include <vector>

#include "../lexer.hpp"
#include "../source_file.hpp"
#include "src/parsers/shared.hpp"

namespace parsers::file {
namespace ast {
struct FunctionArgumentDefinition {
    shared::ast::NodeLocation location;
    shared::ast::NameNode name;
    shared::ast::NameNode type;
};

struct FunctionImperativeDefinition {
    shared::ast::NodeLocation location;
    shared::ast::NameNode name;
    std::map<std::string, FunctionArgumentDefinition> arguments;
};

struct FunctionCompositionDefinition {
    shared::ast::NodeLocation location;
    shared::ast::NameNode name;
    std::vector<shared::ast::NameNode> functions;
};

using FunctionDefinition = std::variant<
    FunctionImperativeDefinition,
    FunctionCompositionDefinition
>;

using Node = std::variant<FunctionDefinition>;
};  // namespace ast

using UnexpectedToken = lexer::Token;

struct UnexpectedEOF {};

using Error = std::variant<UnexpectedToken, UnexpectedEOF>;
using Errors = std::vector<Error>;
using Result = std::vector<ast::Node>;
using ReturnType = std::expected<Result, Errors>;

using GetNextTokenFunc = std::function<std::optional<lexer::Token>()>;

using Func = std::function<ReturnType(
    const GetNextTokenFunc &getNextToken,
    const std::weak_ptr<SourceFile> &sourceFile) noexcept>;

};  // namespace parsers::file
