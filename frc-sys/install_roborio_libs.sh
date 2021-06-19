#!/bin/sh

# Download required roborio libraries from the roborio to be used for linking.
#
# Find required libraries by
# 1) try to compile frc-sys
# 2) see what error comes up about a missing library
# 3) find that library on the roborio
# 4) repeat

libs=(
    "/usr/local/frc/lib/libFRC_NetworkCommunication.so"
    "/usr/local/natinst/lib/libni_emb.so"
    "/usr/local/natinst/lib/libni_rtlog.so"
    "/usr/local/natinst/lib/libNiFpga.so"
    "/usr/local/natinst/lib/libNiFpgaLv.so"
    "/usr/local/natinst/lib/libnirio_emb_can.so"
    "/usr/local/natinst/lib/libniriodevenum.so"
    "/usr/local/natinst/lib/libniriosession.so"
    "/usr/local/natinst/lib/libNiRioSrv.so"
    "/usr/local/frc/lib/libRoboRIO_FRC_ChipObject.so"
    "/usr/local/vxipnp/linux/lib/libvisa.so"
)

lib_dir="lib/roborio/"

for lib in "${libs[@]}"; do
    echo $lib
    scp "lvuser@roborio-846-frc.local:$lib" $lib_dir
done