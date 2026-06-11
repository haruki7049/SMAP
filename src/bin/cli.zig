const std = @import("std");
const flags = @import("flags");

var gpa: std.heap.DebugAllocator(.{}) = .init;
const allocator = gpa.allocator();

const Flags = struct {
    version: bool = false,

    pub const switches = .{
        .version = 'V',
    };
};

pub fn main() !void {
    defer {
        const deinit_status = gpa.deinit();
        if (deinit_status == .leak)
            @panic("Memory Leaked");
    }

    const args = try std.process.argsAlloc(allocator);
    defer std.process.argsFree(allocator, args);

    const cli = flags.parse(args, "SMAP CLI", Flags, .{});
    if (cli.version) {
        var buf: [1024]u8 = undefined;
        var stdout_writer = std.fs.File.stdout().writer(&buf);
        const stdout = &stdout_writer.interface;

        try stdout.print("SMAP CLI version {s}\n", .{"0.0.0"});
        try stdout.flush();
    }
}
