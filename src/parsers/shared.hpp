#pragma once

#include <memory>
#include <string>
#include <vector>

#include "../lexer.hpp"
#include "../source_file.hpp"

namespace parsers::shared::ast {
struct NodeLocation {
    lexer::Token startToken;
    lexer::Token endToken;
    std::weak_ptr<SourceFile> sourceFile;
};

struct NameNode {
    NodeLocation location;
    std::vector<std::string> parts;
};
};  // namespace parsers::shared::ast
