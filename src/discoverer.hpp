#pragma once

#include <functional>
#include <vector>
#include "source_file.hpp"

namespace discoverer {
using DiscovererFunction = std::function<std::vector<SourceFile>() noexcept>;
};
