#pragma once

#include <filesystem>
#include <string>

struct SourceFile {
    std::filesystem::path path;
    std::string buffer;
};
