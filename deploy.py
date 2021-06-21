#!/usr/bin/env python3

"""Deploy executable and libraries to the roborio.

After compiling the robot package, this script finds the roborio address, copies over the executable
and wpilib libraries, and runs several scripts remotely.

positional arguments:
  package            robot package to deploy

optional arguments:
  -h, --help         show this help message and exit
  --verbose, -v      log shell commands
  --skip-wpilib, -s  skip deploying wpilib libraries
  --build, -b        run cargo build automatically before deploying

e.g. `./deploy.py rswerve -sb` build and deploys the rswerve package, skipping wpilib libraries
"""

from argparse import ArgumentParser
import subprocess
import sys
from os import path

EXECUTABLE_DIR = "target/arm-unknown-linux-gnueabi/release"
SHARED_LIB_DIR = "frc-sys/lib/linux/athena/shared"

WPILIB_LIBS = [
    "cameraserver",
    "ntcore",
    "wpiHal",
    "wpilibc",
    "wpimath",
    "wpiutil",
]

ROBORIO_HOME_DIR = "/home/lvuser/"
ROBORIO_LIB_DIR = "/usr/local/frc/third-party/lib"

TEAM_NUMBER = 846


def main():
    # args parsed from command line
    global args
    # roborio address
    global address

    parser = ArgumentParser(description="Deploy code to the roborio.")
    parser.add_argument("package", type=str, help="robot package to deploy")
    parser.add_argument("--verbose", "-v", action="store_true", help="log shell commands")
    parser.add_argument("--skip-wpilib", "-s", action="store_true",
                        help="skip deploying wpilib libraries")
    parser.add_argument("--build", "-b", action="store_true",
                        help="run cargo build automatically before deploying")
    args = parser.parse_args()

    if args.build:
        build_package()

    verify_executable()
    address = find_roborio()

    if not args.skip_wpilib:
        deploy_wpilib_libs()

    run_predeploy()
    deploy_executable()
    run_postdeploy()


def build_package():
    """Run cargo build --release on the robot package."""

    print(f"Building {args.package}")
    cmd(f"cargo build -p {args.package} --release", suppress=False)


def verify_executable():
    """Verify that the robot executable exists.

    The robot package must be built in the release profile.
    """

    exe = path.join(EXECUTABLE_DIR, args.package)
    if not path.exists(exe):
        sys.exit(f"Robot executable {exe} not found.")
    print(f"Executable found at {exe}")


def find_roborio():
    """Find the roborio IP address.

    Attempts to ping several address until success.
    """

    addresses = [
        f"roborio-{TEAM_NUMBER}-frc.local",
        f"10.{TEAM_NUMBER // 100}.{TEAM_NUMBER % 100}.2",
        "172.22.11.2"
    ]

    for address in addresses:
        print(f"Pinging {address}... ", end="", flush=True)
        ping = cmd(f"ping -c 1 {address}")
        if ping.returncode == 0:
            print("Success")
            return address
        else:
            print("Fail")

    sys.exit("Unable to connect to roborio.")


def deploy_wpilib_libs():
    """Copies wpilib libraries to the roborio."""

    for lib in WPILIB_LIBS:
        f = path.join(SHARED_LIB_DIR, f"lib{lib}.so")
        print(f"Deploying vendor library: {f}")
        scp(f, ROBORIO_LIB_DIR)


def run_predeploy():
    """Run predeploy scripts on the roborio.

    Based on https://github.com/wpilibsuite/GradleRIO/blob/HEAD/src/main/groovy/edu/wpi/first/gradlerio/frc/FRCNativeArtifact.groovy
    """

    print("Running predeploy scripts")
    commands = [
        ". /etc/profile.d/natinst-path.sh; /usr/local/frc/bin/frcKillRobot.sh -t 2> /dev/null",
        f"rm -f {path.join(ROBORIO_HOME_DIR, args.package)}"
    ]

    for c in commands:
        cmd_ssh(c)


def deploy_executable():
    """Copies the robot executable to the roborio."""

    f = path.join(EXECUTABLE_DIR, args.package)
    print(f"Deploying executable: {f}")
    cmd_ssh([f"rm -f /home/lvuser/{args.package}"])
    scp(f, ROBORIO_HOME_DIR)


def run_postdeploy():
    """Runs postdeploy scripts on the roborio.

    Based on https://github.com/wpilibsuite/GradleRIO/blob/HEAD/src/main/groovy/edu/wpi/first/gradlerio/frc/FRCNativeArtifact.groovy

    Also updates the robotCommand file in the roborio.
    """

    print("Running postdeploy scripts")
    file = path.join(ROBORIO_HOME_DIR, args.package)
    commands = [
        f"echo \"{file}\" > /home/lvuser/robotCommand",
        "chmod +x /home/lvuser/robotCommand; chown lvuser /home/lvuser/robotCommand",
        f"chmod +x {file}; chown lvuser {file}",
        "sync",
        "ldconfig",
        ". /etc/profile.d/natinst-path.sh; /usr/local/frc/bin/frcKillRobot.sh -t -r 2> /dev/null"
    ]

    for c in commands:
        cmd_ssh(c)


def cmd_ssh(command):
    """Run a command on the roborio."""

    cmd(f"ssh admin@{address} '{command}'")


def scp(source, target):
    """Copy files from local to the roborio."""

    cmd(f"scp {source} admin@{address}:{target}")


def cmd(command, suppress=True):
    """Run a command (suppresses output)."""

    if args.verbose:
        print(command)
    if suppress:
        return subprocess.run(command, shell=True, stdout=subprocess.DEVNULL, stderr=subprocess.DEVNULL)
    else:
        return subprocess.run(command, shell=True)


if __name__ == "__main__":
    try:
        main()
    except KeyboardInterrupt:
        sys.exit("\nCancelled.")
