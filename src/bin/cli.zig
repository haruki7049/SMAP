const std = @import("std");
const flags = @import("flags");
const build_zig_zon = @embedFile("../../build.zig.zon");

const Flags = struct {
    version: bool = false,

    pub const descriptions = .{
        .version = "Displays this CLI's Version",
    };

    pub const switches = .{
        .version = 'V',
    };
};

pub fn main() !void {
    var gpa: std.heap.DebugAllocator(.{}) = .init;
    const allocator = gpa.allocator();

    defer {
        const deinit_status = gpa.deinit();
        if (deinit_status == .leak)
            @panic("Memory Leaked");
    }

    const args = try std.process.argsAlloc(allocator);
    defer std.process.argsFree(allocator, args);

    const cli = flags.parse(args, "SMAP CLI", Flags, .{});
    if (cli.version) {
        // Fetch this project's version
        const version = parse_version(allocator, build_zig_zon);
        defer version.free();

        var buf: [1024]u8 = undefined;
        var stdout_writer = std.fs.File.stdout().writer(&buf);
        const stdout = &stdout_writer.interface;

        try stdout.print("SMAP CLI version {s}\n", .{version.toString()});
        try stdout.flush();
    }
}

const Version = struct {
    const Self = @This();

    allocator: std.mem.Allocator,

    major: usize,
    minor: usize,
    patch: usize,

    pub fn toString(self: Self) std.mem.Allocator.Error![]const u8 {
        return std.fmt.allocPrint(self.allocator, "{d}.{d}.{d}", .{ self.major, self.minor, self.patch });
    }

    pub fn free(self: Self) void {
        std.zon.parse.free(self.allocator, self);
    }
};

fn parse_version(allocator: std.mem.Allocator, bytes: []const u8) Version {
    return try std.zon.parse.fromSlice(
        Version,
        allocator,
        bytes,
        null,
        .{ .ignore_unknown_fields = true, .free_on_error = true },
    );
}
