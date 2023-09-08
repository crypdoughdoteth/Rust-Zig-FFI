const std = @import("std");
const print = std.debug.print;

export fn square(c: ?*i32) ?*i32 {
    if (c) |val| {
        const value = val.*;
        c.?.* = value * value; 
    }  
    return c;
}