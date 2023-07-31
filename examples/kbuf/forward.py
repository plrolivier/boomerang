""" Script illustrating how to use chestburster to forward syscall for a given character device.
"""
import logging
import IPython as ip
import os
from os.path import dirname
from time import sleep

from chestburster import Chestburster



logger = logging.getLogger(__name__)
#log.setLevel('DEBUG')

WD = os.getcwd()
PID = os.getpid()

#BASE_DIR='/tmp/kbuf/'

PROJECT_PATH = dirname(dirname(dirname(__file__)))
BUILD_PATH =  PROJECT_PATH + "/target/debug/"

TRACER_PATH = BUILD_PATH + "ptracer"
TRACER_IP="127.0.0.1"
TRACER_PORT=31000

EXECUTOR_PATH = BUILD_PATH + "executor"
EXECUTOR_IP="127.0.0.1"
EXECUTOR_PORT=31001

PROGRAM = PROJECT_PATH + "/examples/kbuf/test-kbuf"
ARGS = []

# =============================================================================

cb = Chestburster();
executor = cb.init_executor(executable_path=EXECUTOR_PATH, ip=EXECUTOR_IP, port=EXECUTOR_PORT)
tracer = cb.init_ptracer(executable_path=TRACER_PATH, ip=TRACER_IP, port=TRACER_PORT)

# Setup processes
pid1 = executor.spawn_process(PROGRAM, ARGS)
logger.info(f"Executor spawn process: {pid1}")
pid2 = tracer.spawn_process(PROGRAM, ARGS)
logger.info(f"Tracer spawn process: {pid2}")

# Trace syscall
ack = tracer.start_tracing([pid2])

sleep(1)
#ip.embed()

ack = tracer.stop_tracing([pid2])

# Terminate processes
ack = tracer.kill_process([pid2])
ack = executor.kill_process([pid1])

cb.shutdown()
