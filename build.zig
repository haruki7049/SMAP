const std = @import("std");

pub fn build(b: *std.Build) void {
    const target = b.standardTargetOptions(.{});
    const optimize = b.standardOptimizeOption(.{});

    // Library
    const mod = b.addModule("SMAP", .{
        .root_source_file = b.path("src/smap.zig"),
        .target = target,
    });

    // SMAP CLI
    const cli = b.addExecutable(.{
        .name = "smap",
        .root_module = b.createModule(.{
            .root_source_file = b.path("src/bin/cli.zig"),
            .target = target,
            .optimize = optimize,
            .imports = &.{
                .{ .name = "SMAP", .module = mod },
            },
        }),
    });
    b.installArtifact(cli);

    // SMAP Daemon
    const daemon = b.addExecutable(.{
        .name = "smapd",
        .root_module = b.createModule(.{
            .root_source_file = b.path("src/bin/daemon.zig"),
            .target = target,
            .optimize = optimize,
            .imports = &.{
                .{ .name = "SMAP", .module = mod },
            },
        }),
    });
    b.installArtifact(daemon);

    // Tests
    const mod_tests = b.addTest(.{
        .root_module = mod,
    });
    const run_mod_tests = b.addRunArtifact(mod_tests);

    const cli_tests = b.addTest(.{
        .root_module = cli.root_module,
    });
    const run_cli_tests = b.addRunArtifact(cli_tests);

    const daemon_tests = b.addTest(.{
        .root_module = daemon.root_module,
    });
    const run_daemon_tests = b.addRunArtifact(daemon_tests);

    // Run tests
    const test_step = b.step("test", "Run tests");
    test_step.dependOn(&run_mod_tests.step);
    test_step.dependOn(&run_cli_tests.step);
    test_step.dependOn(&run_daemon_tests.step);

    // Just like flags, top level steps are also listed in the `--help` menu.
    //
    // The Zig build system is entirely implemented in userland, which means
    // that it cannot hook into private compiler APIs. All compilation work
    // orchestrated by the build system will result in other Zig compiler
    // subcommands being invoked with the right flags defined. You can observe
    // these invocations when one fails (or you pass a flag to increase
    // verbosity) to validate assumptions and diagnose problems.
    //
    // Lastly, the Zig build system is relatively simple and self-contained,
    // and reading its source code will allow you to master it.
}
