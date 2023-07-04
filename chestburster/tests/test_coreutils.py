""" Provide tests for coreutils programs.

+-----------------------------+-----------+----------+
| Categorie                   | Name      | tested ? |
+-----------------------------+-----------+----------+
| Output of entire files      | cat       | n        |
|                             | tac       | y        |
|                             | nl        | y        |
|                             | od        | y        |
|                             | base32    | y        |
|                             | base64    | y        |
|                             | basenc    | y        |
+-----------------------------+-----------+----------+
| Formatting file contents    | fmt       |          |
|                             | pr        |          |
|                             | fold      |          |
+-----------------------------+-----------+----------+
| Output of parts of files    | head      | y        |
|                             | tail      | y        |
|                             | split     |          |
|                             | csplit    |          |
+-----------------------------+-----------+----------+
| Summarizing files           | wc        | y        |
|                             | sum       | y        |
|                             | cksum     | y        |
|                             | b2sum     | y        |
|                             | md5sum    | y        |
|                             | sha1sum   | y        |
|                             | sha224sum | y        |
|                             | sha256sum | y        |
|                             | sha384sum | y        |
|                             | sha512sum | y        |
+-----------------------------+-----------+----------+
| Operating on sorted files   | sort      |          |
|                             | shuf      |          |
|                             | uniq      |          |
|                             | comm      |          |
|                             | ptx       |          |
|                             | tsort     |          |
+-----------------------------+-----------+----------+
| Operating on fields         | cut       |          |
|                             | paste     |          |
|                             | join      |          |
+-----------------------------+-----------+----------+
| Operating on characters     | tr        |          |
|                             | expand    |          |
|                             | unexpand  |          |
+-----------------------------+-----------+----------+
| Directory listing           | ls        | y        |
|                             | dir       | y        |
|                             | vdir      | y        |
|                             | dircolors | y        |
+-----------------------------+-----------+----------+
| Basic operations            | cp        |          |
|                             | dd        |          |
|                             | ginstall  |          |
|                             | mv        |          |
|                             | rm        |          |
|                             | shred     |          |
+-----------------------------+-----------+----------+
| Special file types          | link      |          |
|                             | ln        |          |
|                             | mkdir     |          |
|                             | mkfifo    |          |
|                             | mknod     |          |
|                             | readlink  |          |
|                             | rmdir     |          |
|                             | unlink    |          |
+-----------------------------+-----------+----------+
| Changing file attributes    | chown     |          |
|                             | chgrp     |          |
|                             | chmod     |          |
|                             | touch     |          |
+-----------------------------+-----------+----------+
| File space usage            | df        | y        |
|                             | du        | y        |
|                             | stat      |          |
|                             | sync      | y        |
|                             | truncate  |          |
+-----------------------------+-----------+----------+
| Printing text               | echo      |          |
|                             | printf    |          |
|                             | yes       |          |
+-----------------------------+-----------+----------+
| Conditions                  | false     |          |
|                             | true      |          |
|                             | test      |          |
|                             | expr      |          |
+-----------------------------+-----------+----------+
| Redirection                 | tee       |          |
+-----------------------------+-----------+----------+
| File name manipulation      | basename  |          |
|                             | dirname   |          |
|                             | pathchk   |          |
|                             | mktemp    |          |
|                             | realpath  |          |
+-----------------------------+-----------+----------+
| Working context             | pwd       | y        |
|                             | stty      | y        |
|                             | printenv  | y        |
|                             | tty       | y        |
+-----------------------------+-----------+----------+
| User information            | id        | y        |
|                             | logname   | y        |
|                             | whoami    | y        |
|                             | groups    | y        |
|                             | users     | y        |
|                             | who       | y        |
+-----------------------------+-----------+----------+
| System context              | date      | y        |
|                             | arch      | n        |
|                             | nproc     | y        |
|                             | uname     | y        |
|                             | hostname  | n        |
|                             | hostid    | y        |
|                             | uptime    | y        |
+-----------------------------+-----------+----------+
| SELinux context             | chcon     |          |
|                             | runcon    |          |
+-----------------------------+-----------+----------+
| Modified command invocation | chroot    |          |
|                             | env       |          |
|                             | nice      |          |
|                             | nohup     |          |
|                             | stdbuf    |          |
|                             | timeout   |          |
+-----------------------------+-----------+----------+
| Process control             | kill      |          |
+-----------------------------+-----------+----------+
| Delaying                    | sleep     |          |
+-----------------------------+-----------+----------+
| Numeric operations          | factor    |          |
|                             | numfmt    |          |
|                             | seq       |          |
+-----------------------------+-----------+----------+
|                             | [         |          |
|                             | pinky     |          |
+-----------------------------+-----------+----------+

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
# Output of entire files

class TestOutputEntireFiles(TestForward):

    file_path = '/tmp/random.data'
    
    @classmethod
    def setUpClass(cls):
        setup_file_by_size(cls.file_path, 1024)

    @classmethod
    def tearDownClass(cls):
        clean_file(cls.file_path)

class TestTac(TestOutputEntireFiles):

    path = '/bin/tac'

    def test_no_args(self):
        self.forward(self.path, [self.file_path])

class TestNl(TestOutputEntireFiles):

    path = '/bin/nl'

    def test_no_args(self):
        self.forward(self.path, [self.file_path])

class TestOd(TestOutputEntireFiles):

    path = '/bin/od'

    def test_no_args(self):
        self.forward(self.path, [self.file_path])

    def test_address_radix(self):
        self.forward(self.path, ['-A', self.file_path])

class TestBase32(TestOutputEntireFiles):

    path = '/bin/base32'

    def test_no_args(self):
        self.forward(self.path, [self.file_path])

class TestBase64(TestOutputEntireFiles):

    path = '/bin/base64'

    def test_no_args(self):
        self.forward(self.path, [self.file_path])

class TestBasenc(TestOutputEntireFiles):

    path = '/bin/basenc'

    def test_no_args(self):
        self.forward(self.path, [self.file_path])

# =============================================================================
# Formatting file contents

# =============================================================================
# Output of parts of files

class TestOutputPartsFile(TestForward):

    file_path = '/tmp/random.data'
    
    @classmethod
    def setUpClass(cls):
        setup_file_by_size(cls.file_path, 1024)

    @classmethod
    def tearDownClass(cls):
        clean_file(cls.file_path)

class TestHead(TestOutputPartsFile):

    path = '/bin/head'

    def test_no_args(self):
        self.forward(self.path, [self.file_path])

    def test_lines(self):
        self.forward(self.path, ['-n 10', self.file_path])

    def test_bytes(self):
        self.forward(self.path, ['-c 10', self.file_path])

class TestTail(TestOutputPartsFile):

    path = '/bin/tail'

    def test_no_args(self):
        self.forward(self.path, [self.file_path])

    def test_lines(self):
        self.forward(self.path, ['-n 10', self.file_path])

    def test_bytes(self):
        self.forward(self.path, ['-c 10', self.file_path])

#class TestSplit(TestOutputPartsFile):
#
#    path = '/bin/split'
#
#    def test_no_args(self):
#        self.forward(self.path, [self.file_path])
#
#class TestCsplit(TestOutputPartsFile):
#
#    path = '/bin/csplit'
#
#    def test_no_args(self):
#        self.forward(self.path, [self.file_path])

# =============================================================================
# Summarizing files

class TestSummarizingFiles(TestForward):

    file_path = '/tmp/random.data'
    
    @classmethod
    def setUpClass(cls):
        setup_file_by_size(cls.file_path, 1024)

    @classmethod
    def tearDownClass(cls):
        clean_file(cls.file_path)


class TestWc(TestSummarizingFiles):

    path = '/bin/wc'

    def test_no_args(self):
        self.forward(self.path, [self.file_path])

    def test_bytes(self):
        self.forward(self.path, ['--bytes', self.file_path])

    def test_chars(self):
        self.forward(self.path, ['--chars', self.file_path])

    def test_lines(self):
        self.forward(self.path, ['--lines', self.file_path])

    def test_words(self):
        self.forward(self.path, ['--words', self.file_path])

class TestSum(TestSummarizingFiles):

    path = '/bin/sum'

    def test_no_args(self):
        self.forward(self.path, [self.file_path])

class TestCksum(TestSummarizingFiles):

    path = '/bin/cksum'

    def test_no_args(self):
        self.forward(self.path, [self.file_path])

class TestB2sum(TestSummarizingFiles):

    path = '/bin/b2sum'

    def test_no_args(self):
        self.forward(self.path, [self.file_path])

class Testmd5sum(TestSummarizingFiles):

    path = '/bin/md5sum'

    def test_no_args(self):
        self.forward(self.path, [self.file_path])

class Testsha1sum(TestSummarizingFiles):

    path = '/bin/sha1sum'

    def test_no_args(self):
        self.forward(self.path, [self.file_path])

class Testsha224sum(TestSummarizingFiles):

    path = '/bin/sha224sum'

    def test_no_args(self):
        self.forward(self.path, [self.file_path])

class Testsha256sum(TestSummarizingFiles):

    path = '/bin/sha256sum'

    def test_no_args(self):
        self.forward(self.path, [self.file_path])

class Testsha384sum(TestSummarizingFiles):

    path = '/bin/sha384sum'

    def test_no_args(self):
        self.forward(self.path, [self.file_path])

class Testsha512sum(TestSummarizingFiles):

    path = '/bin/sha512sum'

    def test_no_args(self):
        self.forward(self.path, [self.file_path])


# =============================================================================
# Operating on sorted files

# =============================================================================
# Operating on fields

# =============================================================================
# Operating on characters

# =============================================================================
# Directory listing

class TestDirectoryListing(TestForward):

    @classmethod
    def setUpClass(cls):
        setup_files(BASE_DIR)

    @classmethod
    def tearDownClass(cls):
        clean_files(BASE_DIR)

class TestLs(TestDirectoryListing):

    path = '/bin/ls'

    def test_no_args(self):
        self.forward(self.path, [])

    def test_tmp(self):
        self.forward(self.path, [BASE_DIR])
    
    def test_directory(self):
        self.forward(self.path, ['-d', BASE_DIR])

    def test_size(self):
        self.forward(self.path, ['--size', BASE_DIR])

    def test_time(self):
        self.forward(self.path, ['-t', BASE_DIR])

    def test_long(self):
        # long + size + inode
        self.forward(self.path, ['-lsi', BASE_DIR])

    def test_recursive(self):
        self.forward(self.path, ['--recursive', BASE_DIR])

    def test_long_recursive(self):
        self.forward(self.path, ['-l', '--recursive', BASE_DIR])

class TestDir(TestLs):

    path = '/bin/dir'

class TestVdir(TestLs):

    path = '/bin/vdir'

class TestDircolors(TestForward):

    path = '/bin/dircolors'

    def test_no_args(self):
        self.forward(self.path, [])

    def test_print_database(self):
        self.forward(self.path, ['--print-database'])

    def test_print_ls_colors(self):
        self.forward(self.path, ['--print-ls-colors'])


# =============================================================================
# Basic operations

# =============================================================================
# Special file types

# =============================================================================
# Changing file attributes

# =============================================================================
# File space usage

class TestDu(TestForward):

    path = '/bin/du'

    def test_no_args(self):
        self.forward(self.path, [])

    def test_all(self):
        self.forward(self.path, ['--all'])

    def test_summarize(self):
        self.forward(self.path, ['--summarize'])

class TestDf(TestForward):

    path = '/bin/df'

    def test_no_args(self):
        self.forward(self.path, [])

    def test_all(self):
        self.forward(self.path, ['--all'])

class TestSync(TestForward):

    path = '/bin/sync'

    def test_no_args(self):
        self.forward(self.path, [])

    def test_all(self):
        self.forward(self.path, ['--data'])

# =============================================================================
# Printing text

# =============================================================================
# Conditions

# =============================================================================
# Redirection

# =============================================================================
# File name manipulation

# =============================================================================
# Woking context

class TestPwd(TestForward):

    path = '/bin/pwd'

    def test_no_args(self):
        self.forward(self.path, [])

class TestStty(TestForward):

    path = '/bin/stty'

    def test_no_args(self):
        self.forward(self.path, [])

    def test_all(self):
        self.forward(self.path, ['--all'])

class TestPrintenv(TestForward):

    path = '/bin/printenv'

    def test_no_args(self):
        self.forward(self.path, [])

class TestTty(TestForward):

    path = '/bin/tty'

    def test_no_args(self):
        self.forward(self.path, [])

# =============================================================================
# User information

class TestWhoami(TestForward):

    def test_no_args(self):
        self.forward('/bin/whoami', [])

class TestUsers(TestForward):

    def test_no_args(self):
        self.forward('/bin/users', [])

class TestWho(TestForward):

    def test_no_args(self):
        self.forward('/bin/who', [])

    def test_all(self):
        self.forward('/bin/who', ['--all'])

class TestGroups(TestForward):

    def test_no_args(self):
        self.forward('/bin/groups', [])

class TestId(TestForward):

    path = '/bin/id'

    def test_no_args(self):
        self.forward(self.path, [])

class TestLogname(TestForward):

    path = '/bin/logname'

    def test_no_args(self):
        self.forward(self.path, [])


# =============================================================================
# System context

class TestDate(TestForward):

    path = '/bin/date'

    def test_no_args(self):
        self.forward(self.path, [])

    def test_debug(self):
        self.forward(self.path, ['--debug'])

class TestNproc(TestForward):

    path = '/bin/nproc'

    def test_no_args(self):
        self.forward(self.path, [])

    def test_all(self):
        self.forward(self.path, ['--all'])

class TestUname(TestForward):

    path = '/bin/uname'

    def test_no_args(self):
        self.forward(self.path, [])

    def test_all(self):
        self.forward(self.path, ['--data'])

class TestHostid(TestForward):

    path = '/bin/hostid'

    def test_no_args(self):
        self.forward(self.path, [])

class TestUptime(TestForward):

    path = '/bin/uptime'

    def test_no_args(self):
        self.forward(self.path, [])

    def test_pretty(self):
        self.forward(self.path, ['--pretty'])


# =============================================================================
# SELinux context

# =============================================================================
# Modified command invocation

# =============================================================================
# Process control

# =============================================================================
# Delaying

# =============================================================================
# Numeric operations





# =============================================================================
if __name__ == '__main__':
    unittest.main()
