""" Provide tests for coreutils programs.
"""

#import sys
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

#from avatar2.targets import TargetStates

from chestburster import Chestburster

from config import *



logger = logging.getLogger(__name__)
#log.setLevel('DEBUG')

WD = os.getcwd()
PID = os.getpid()

# =============================================================================

class TestForward(unittest.TestCase):

    def forward(self, program, args=[]):
        cb = Chestburster();
        executor = cb.init_executor(executable_path=EXECUTOR_PATH, ip=EXECUTOR_IP, port=EXECUTOR_PORT)
        tracer = cb.init_ptracer(executable_path=TRACER_PATH, ip=TRACER_IP, port=TRACER_PORT)

        # Setup processes
        pid1 = executor.spawn_process(program, args)
        logger.info(f"Executor spawn process: {pid1}")
        pid2 = tracer.spawn_process(program, args)
        logger.info(f"Tracer spawn process: {pid2}")

        # Trace syscall
        ack = tracer.start_tracing([pid2])
        #self.assertEqual(ack, b'ACK', "Fail to start tracing")

        #ip.embed()

        ack = tracer.stop_tracing([pid2])
        #self.assertEqual(ack, b'ACK', "Fail to stop tracing")

        # Terminate processes
        ack = tracer.kill_process([pid2])
        #self.assertEqual(ack, b'ACK', "Fail to kill process on tracer side")
        ack = executor.kill_process([pid1])
        #self.assertEqual(ack, b'ACK', "Fail to kill process on executor side")

        cb.shutdown()

# =============================================================================

class TestLs(TestForward):

    @classmethod
    def setUpClass(cls):
        setup_files(BASE_DIR)

    @classmethod
    def tearDownClass(cls):
        clean_files(BASE_DIR)


    def test_no_args(self):
        self.forward('/bin/ls', [])

    #def test_root(self):
    #    self.forward('/bin/ls', ['/'])

    def test_tmp(self):
        self.forward('/bin/ls', [BASE_DIR])
    
    def test_directory(self):
        self.forward('/bin/ls', ['-d', BASE_DIR])

    def test_size(self):
        self.forward('/bin/ls', ['--size', BASE_DIR])

    def test_time(self):
        self.forward('/bin/ls', ['-t', BASE_DIR])

    def test_long(self):
        # long + size + inode
        self.forward('/bin/ls', ['-lsi', BASE_DIR])

    def test_recursive(self):
        self.forward('/bin/ls', ['--recursive', BASE_DIR])

    def test_long_recursive(self):
        self.forward('/bin/ls', ['-l', '--recursive', BASE_DIR])

# =============================================================================

class TestWho(TestForward):

    def test_no_args(self):
        self.forward('/bin/who', [])

    def test_all(self):
        self.forward('/bin/who', ['--all'])

# =============================================================================

class TestWhoami(TestForward):

    def test_no_args(self):
        self.forward('/bin/whoami', [])

# =============================================================================

class TestDf(TestForward):

    def test_no_args(self):
        self.forward('/bin/df', [])

    def test_all(self):
        self.forward('/bin/df', ['--all'])

# =============================================================================

class TestDu(TestForward):

    def test_no_args(self):
        self.forward('/bin/du', [])

    def test_all(self):
        self.forward('/bin/du', ['--all'])

    def test_summarize(self):
        self.forward('/bin/du', ['--summarize'])

# =============================================================================

class TestSync(TestForward):

    def test_no_args(self):
        self.forward('/bin/sync', [])

    def test_all(self):
        self.forward('/bin/sync', ['--data'])






# =============================================================================
if __name__ == '__main__':
    unittest.main()
