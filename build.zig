const std = @import("std");

pub fn build(b: *std.Build) void {
    const target = b.standardTargetOptions(.{});
    const optimize = b.standardOptimizeOption(.{});

    // Library
    const mod = b.addModule("SMAP", .{
        .root_source_file = b.path("src/smap.zig"),
        .target = target,
    });

    // Run step
    const run_step = b.step("run", "Run the default app");

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
    const run_cli_step = add_run_step(b, cli, "run-cli", "Run the SMAP CLI");
    run_step.dependOn(run_cli_step); // The default app is the SMAP CLI

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
    _ = add_run_step(b, daemon, "run-daemon", "Run the SMAP Daemon");

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
}

fn add_run_step(
    b: *std.Build,
    exe: *std.Build.Step.Compile,
    name: []const u8,
    description: []const u8,
) *std.Build.Step {
    var run_exe_step = b.step(name, description);
    const run_cmd = b.addRunArtifact(exe);
    run_exe_step.dependOn(&run_cmd.step);
    run_cmd.step.dependOn(b.getInstallStep());
    if (b.args) |args| {
        run_cmd.addArgs(args);
    }

    return run_exe_step;
}
