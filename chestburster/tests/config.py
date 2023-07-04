"""
"""
import unittest
import os
import logging
import signal
import IPython as ip
import pprint
import random
import string
import shutil
import socket
import stat

from time import time, sleep
from os.path import dirname



# =============================================================================

BASE_DIR='/tmp/cb_pytest/'

BUILD_PATH = dirname(dirname(dirname(__file__))) + "/target/debug/"

TRACER_PATH = BUILD_PATH + "ptracer"
TRACER_IP="127.0.0.1"
TRACER_PORT=31000

EXECUTOR_PATH = BUILD_PATH + "executor"
EXECUTOR_IP="127.0.0.1"
EXECUTOR_PORT=31001


# =============================================================================

def setup_file_by_size(path=None, size=1024):
    """ Create a file of size bytes
    """

    PATH = path if path is not None else '/tmp/random_file.data'
    with open(PATH, 'wb') as f:
        content = bytearray(random.getrandbits(8) for _ in range(size))
        f.write(content)

def clean_file(path):
    os.remove(path)


def setup_files(base_dir=BASE_DIR):
    """ Setupt a directory with different type of files and directory layouts.
    """

    TEXT_SIZE=500

    # Create the base directory
    if not os.path.exists(base_dir):
        os.makedirs(base_dir)

    # Create regular file
    REGULAR_FILE = base_dir + 'regular_file.txt'
    with open(REGULAR_FILE, 'w') as f:
        text = random.choices(string.printable, k=TEXT_SIZE)
        f.write(''.join(text))

    # Create a subdirectory
    SUB_DIR = os.path.join(base_dir, 'subdir/')
    os.makedirs(SUB_DIR)

    # Create a symbolic link to the regular file
    SYMLINK_PATH = os.path.join(base_dir, 'symlink.txt')
    os.symlink(REGULAR_FILE, SYMLINK_PATH)

    # Create a named pipe
    PIPE_PATH = os.path.join(base_dir, 'named_pipe')
    os.mkfifo(PIPE_PATH)

    # Create a socket
    SOCKET_PATH = os.path.join(base_dir, 'socket')
    sk = socket.socket(socket.AF_UNIX, socket.SOCK_STREAM)
    sk.bind(SOCKET_PATH)

    # Do not work unless root:

    # Create a character device
    #CHARDEV_PATH = os.path.join(base_dir, 'char_dev')
    #os.mknod(CHARDEV_PATH, mode=0o660 | stat.S_IFCHR, device=os.makedev(1, 3))

    # Create a block device
    #BLOCKDEV_PATH = os.path.join(base_dir, 'block_dev')
    #os.mknod(BLOCKDEV_PATH, mode=0o660 | stat.S_IFBLK, device=os.makedev(8, 1))

    # Create a FIFO (named pipe)
    FIFO_PATH = os.path.join(base_dir, 'fifo')
    os.mkfifo(FIFO_PATH)

    # Create some recursives directories
    def create_recursive_dir(base_dir, depth):
        if depth <= 0:
            return
        
        # Create the current directory
        current_dir = os.path.join(base_dir, f"dir{depth}")
        os.makedirs(current_dir)

        # Create a file in the current directory
        file_path = os.path.join(current_dir, f"file{depth}.txt")
        with open(file_path, 'w') as f:
            f.write(f"This is file {depth}")
        
        # Recursively create subdirectories
        create_recursive_dir(current_dir, depth - 1)

    create_recursive_dir(base_dir, 2)


def clean_files(base_dir=BASE_DIR):
        shutil.rmtree(base_dir)

