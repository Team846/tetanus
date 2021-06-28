#!/usr/bin/env python3

"""Deploy executable and libraries to the roborio.

Referenced off of gradlerio (download an example C++ project and run gradle deploy to see the logs)

positional arguments:
  package            robot package to deploy

optional arguments:
  -h, --help         show this help message and exit
  --skip-libs, -s    skip deploying shared libraries
  --build, -b        run cargo build before deploying
  --debug, -d        don't add --release flag to cargo build

e.g. `./deploy.py rswerve -sb` build and deploys the rswerve package, skipping wpilib libraries
"""

from argparse import ArgumentParser
import subprocess
import sys
from os import path

EXECUTABLE_DIR = "target/arm-unknown-linux-gnueabi"
SHARED_LIB_DIR = "frc-sys/lib/linux/athena/shared"

WPILIB_LIBS = [
    "cameraserver",
    "cscore",
    "ntcore",
    "wpiHal",
    "wpilibc",
    "wpimath",
    "wpiutil",
]

THIRD_PARTY_LIBS = [
    "opencv_stitching",
    "opencv_videoio",
    "opencv_flann",
    "opencv_video",
    "opencv_imgcodecs",
    "opencv_highgui",
    "opencv_objdetect",
    "opencv_imgproc",
    "opencv_calib3d",
    "opencv_core",
    "opencv_features2d",
    "opencv_photo",
    "opencv_ml",
    "opencv_shape",
    "opencv_superres",
    "opencv_videostab",
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
    parser.add_argument("--skip-libs", "-s", action="store_true",
                        help="skip deploying shared libraries")
    parser.add_argument("--build", "-b", action="store_true",
                        help="run cargo build before deploying")
    parser.add_argument("--debug", "-d", action="store_true",
                        help="don't add --release flag to cargo build")
    args = parser.parse_args()

    #####

    # run cargo build, unless -b is passed
    if args.build:
        cmd(f"cargo build -p {args.package} {'' if args.debug else '--release'}",
            suppress=False)

    print(f"\n\nDeploying {args.package}")
    print("(C: running command on robot, F: copying file to robot)\n\n")

    # path to executable
    exe = path.join(EXECUTABLE_DIR, "debug" if args.debug else "release", args.package)

    # verify executable exists
    if not path.exists(exe):
        sys.exit(f"Robot executable {exe} not found.")

    # find roborio address
    address = find_roborio()

    cmd_ssh("sed -i -e 's/^StartupDLLs/;StartupDLLs/' /etc/natinst/share/ni-rt.ini")

    # deploy shared libraries, unless -s is passed
    if not args.skip_libs:
        for lib in WPILIB_LIBS:
            if args.debug:
                f = path.join(SHARED_LIB_DIR, f"lib{lib}d.so")
            else:
                f = path.join(SHARED_LIB_DIR, f"lib{lib}.so")
            scp(f, ROBORIO_LIB_DIR)

        for lib in THIRD_PARTY_LIBS:
            scp(path.join(SHARED_LIB_DIR, f"lib{lib}.so"), ROBORIO_LIB_DIR)

        cmd_ssh("chmod -R 777 \"/usr/local/frc/third-party/lib\" || true; chown -R lvuser:ni \"/usr/local/frc/third-party/lib\"")
        cmd_ssh("ldconfig")

    # kill robot and delete previous executable on robot
    cmd_ssh(". /etc/profile.d/natinst-path.sh; /usr/local/frc/bin/frcKillRobot.sh -t 2> /dev/null")
    cmd_ssh("rm -f \"/home/lvuser/frcUserProgram\"")

    # copy new executable to robot (named frcUserProgram)
    scp(exe, path.join(ROBORIO_HOME_DIR, "frcUserProgram"))

    cmd_ssh("echo ' \"/home/lvuser/frcUserProgram\" ' > /home/lvuser/robotCommand")
    cmd_ssh("chmod +x /home/lvuser/robotCommand; chown lvuser /home/lvuser/robotCommand")
    cmd_ssh("chmod +x /home/lvuser/frcUserProgram; chown lvuser /home/lvuser/frcUserProgram")
    cmd_ssh("chmod +x \"/home/lvuser/frcUserProgram\"; chown lvuser \"/home/lvuser/frcUserProgram\"")
    cmd_ssh("setcap cap_sys_nice+eip \"/home/lvuser/frcUserProgram\"")
    cmd_ssh("sync")
    cmd_ssh("ldconfig")
    cmd_ssh(". /etc/profile.d/natinst-path.sh; /usr/local/frc/bin/frcKillRobot.sh -t -r 2> /dev/null")

    print("\n\nDone")


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
        if cmd(f"ping -c 1 {address}"):
            print("Success")
            return address
        else:
            print("Fail")

    sys.exit("Unable to connect to roborio.")


def cmd_ssh(command):
    """Run a command on the roborio."""

    print(f"C: {command}")
    cmd(f"ssh admin@{address} \"{command}\"")


def scp(source, target):
    """Copy files from local to the roborio."""

    print(f"F: {source} -> {target}")
    cmd(f"scp {source} admin@{address}:{target}")


def cmd(command, suppress=True):
    """Run a command."""

    if suppress:
        result = subprocess.run(command, shell=True,
                                stdout=subprocess.DEVNULL, stderr=subprocess.DEVNULL)
    else:
        result = subprocess.run(command, shell=True)

    if result.returncode != 0:
        sys.exit(f"Previous command exited with code {result.returncode}")

    return result


if __name__ == "__main__":
    try:
        main()
    except KeyboardInterrupt:
        sys.exit("\nCancelled.")
