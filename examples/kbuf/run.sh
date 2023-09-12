#!/bin/sh
#
# This script loads the kernel module, start the python forward.py script and unload the kernel module.

set -e


# Check script is run with root
if [ "$UID" -ne 0 ]; then
  echo "This script must be run with sudo or as the root user."
  exit 1
fi

# Check if the module file exists
if [ ! -f "kbuf.ko" ]; then
  echo "Error: The kernel module kbuf.ko does not exist. Try to compile it with make."
  exit 1
fi

# Check if the module's kernel version matches the running kernel version
kernel_version="$(modinfo "kbuf.ko" | awk -F ':' '/^vermagic/{print $2}' | cut -d' ' -f8)"
#echo $kernel_version

running_kernel_version="$(uname -r)"
#echo $running_kernel_version

if [ "$kernel_version" != "$running_kernel_version" ]; then
  echo "The module is NOT compatible with the running kernel."
  exit 1
fi


# Launch the example
insmod ./kbuf.ko
python forward.py
rmmod kbuf