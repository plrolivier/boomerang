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

    # Initialize
    cb = Chestburster();
    executor = cb.init_executor(executable_path=EXECUTOR_PATH, ip=EXECUTOR_IP, port=EXECUTOR_PORT)
    tracer = cb.init_ptracer(executable_path=TRACER_PATH, ip=TRACER_IP, port=TRACER_PORT)

    # Setup processes
    pid1 = executor.spawn_process(program, args)
    print(f"Executor spawn process: {pid1}")
    #sleep(0.5)
    pid2 = tracer.spawn_process(program, args)
    print(f"Tracer spawn process: {pid2}")

    sleep(1)

    # Trace syscall
    ack = tracer.start_tracing([pid2])
    print(f"Start tracing: {ack}")

    sleep(1)
    #ip.embed()

    ack = tracer.stop_tracing([pid2])
    print(f"Stop tracing: {ack}")

    #sleep(1)

    # Terminate processes
    ack = tracer.kill_process([pid2])
    print(f"Kill process {pid2}: {ack}")
    #sleep(0.5)
    ack = executor.kill_process([pid1])
    print(f"Kill process {pid1}: {ack}")

    cb.shutdown()



if __name__ == '__main__':
    parser = argparse.ArgumentParser()
    parser.add_argument('program', help='Program to instrument')
    parser.add_argument('arguments', nargs='*', help='Arguments for the program')
    args = parser.parse_args()

    forward(args.program, args.arguments)