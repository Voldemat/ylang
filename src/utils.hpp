#pragma once

#include <format>
#include <variant>
template <class... Ts>
struct overloaded : Ts... {
    using Ts::operator()...;
};
template <class... Ts>
overloaded(Ts...) -> overloaded<Ts...>;

namespace std {
template <typename... Ts>
struct formatter<variant<Ts...>> {
    template <typename ParseContext>
    constexpr auto parse(ParseContext &ctx) {
        return ctx.begin();
    }

    template <typename FormatContext>
    auto format(const variant<Ts...> &record, FormatContext &ctx) const {
        return std::visit(
            [&ctx](auto &&arg) { return std::format_to(ctx.out(), "{}", arg); },
            record);
    }
};
};  // namespace std
