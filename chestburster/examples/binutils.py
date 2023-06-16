""" This script aims to provide an example on how to use chestburster with the libsysforward to forward system calls from the binutils binaries.
"""

import os
import logging
import argparse
from time import sleep, time
import IPython as ip
import pprint

from os import getcwd
from os.path import dirname


from chestburster.chestburster import Chestburster


BUILD_PATH = dirname(dirname(dirname(__file__))) + "/target/debug/"

TRACER_PATH = BUILD_PATH + "ptracer"
TRACER_IP="127.0.0.1"
TRACER_PORT=31000

EXECUTOR_PATH = BUILD_PATH + "executor"
EXECUTOR_IP="127.0.0.1"
EXECUTOR_PORT=31001

logger = logging.getLogger('chestburster')
logger.setLevel('DEBUG')

def forward(program, args):

    args = ['arg1', 'arg2']

    # Initialize
    cb = Chestburster();
    #executor = cb.init_executor(executable_path=EXECUTOR_PATH, ip=EXECUTOR_IP, port=EXECUTOR_PORT)
    tracer = cb.init_ptracer(executable_path=TRACER_PATH, ip=TRACER_IP, port=TRACER_PORT)

    # Setup processes
    sleep(1)
    #pid1 = executor.spawn_process('/path/one', args)
    #print(pid1)
    pid = tracer.spawn_process('/path/two', args)
    print(f"Spawn process: {pid}")

    # Trace syscall
    sleep(1)
    ack = tracer.start_tracing([pid])
    print(f"Start tracing: {ack}")

    sleep(1)
    ack = tracer.stop_tracing([pid])
    print(f"Stop tracing: {ack}")

    # Terminate processes
    sleep(1)
    ack = tracer.kill_process([pid])
    print(f"Kill process {pid}: {ack}")

    #executor.kill_process([pid1])

    cb.shutdown()



if __name__ == '__main__':
    parser = argparse.ArgumentParser()
    parser.add_argument('program', help='Program to instrument')
    parser.add_argument('arguments', nargs='*', help='Arguments for the program')
    args = parser.parse_args()

    forward(args.program, args.arguments)