#!/usr/bin/env lua

local MAX_ITERANTS = 32 -- :(

local f = assert(io.open("src/unfolded_macros.rs", "wb"))

f:write([[
//! This module is automatically generated. Please edit unfolded_macro_gen.lua
//! instead. Or better yet, replace these with procmacros. (TODO)

#[macro_export]
macro_rules! ecs_iter_accessors {
]])
for n = 1, MAX_ITERANTS do
    f:write("    ($mode1:ident $comp1:ty")
    for m = 2, n do
        f:write(", $mode",m,":ident $comp",m,":ty")
    end
    f:write(") => {[\n")
    for m=1, n do
        f:write("        $crate::ecs_iter_accessor!($mode",m," $comp",m,")")
        if m ~= n then f:write(",") end
        f:write("\n")
    end
    f:write("    ]};\n")
end
f:write([[
}

#[macro_export]
macro_rules! ecs_iterated_unfold {
]])
for n = 1, MAX_ITERANTS do
    f:write("    ($eid:expr, $array:expr, $mode1:ident $comp1:ty")
    for m = 2, n do
        f:write(", $mode",m,":ident $comp",m,":ty")
    end
    f:write(") => {unsafe {\n")
    f:write("        let mut iter = $array.into_iter();\n")
    f:write("        (\n")
    f:write("            $eid,\n")
    for m=1, n do
        f:write("            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode",m," $comp",m,")")
        if m ~= n then f:write(",") end
        f:write("\n")
    end
    f:write("        )}};\n")
end
f:write([[
}

#[macro_export]
macro_rules! ecs_gotten_unfold {
]])
for n = 1, MAX_ITERANTS do
    f:write("    ($array:expr, $mode1:ident $comp1:ty")
    for m = 2, n do
        f:write(", $mode",m,":ident $comp",m,":ty")
    end
    f:write(") => {unsafe {\n")
    f:write("        let mut iter = $array.into_iter();\n")
    f:write("        (\n")
    for m=1, n do
        f:write("            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode",m," $comp",m,")")
        if m ~= n then f:write(",") end
        f:write("\n")
    end
    f:write("        )}};\n")
end
f:write([[
}
]])