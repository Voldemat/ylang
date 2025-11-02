#pragma once

#include <memory>

#include "../lexer.hpp"
#include "../source_file.hpp"

namespace parsers {
namespace ast {
struct NodeLocation {
    lexer::Token startToken;
    lexer::Token endToken;
    std::weak_ptr<SourceFile> sourceFile;
};
};  // namespace ast
};  // namespace parsers
