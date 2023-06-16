import os
import logging
#import subprocess as sp
#import sys
#import gzip
#import json
#import shutil
#import io

from avatar2 import Avatar
from avatar2.archs import X86
#from avatar2.watchmen import watch

from .logger import StdoutFormatter
from .targets import TracerTarget, ExecutorTarget


log = logging.getLogger(__name__)

DEFAULT_OUTPUT_DIR = '/tmp/cb'



class Chestburster(Avatar):
    """ Chestburster enrich avatar2 with support for Linux process and control over its own debuggers.
    """

    def __init__(self, arch=X86,
                 log_to_stdout=True, configure_logging=True, debug=True):

        if not os.path.exists(DEFAULT_OUTPUT_DIR):
            os.makedirs(f'{DEFAULT_OUTPUT_DIR}')
            #os.makedirs(f'{self.DEFAULT_OUTPUT_DIR}/dumps')
            #os.makedirs(f'{self.DEFAULT_OUTPUT_DIR}/maps')

        if configure_logging:
            log_file = f'{DEFAULT_OUTPUT_DIR}/chestburster.log'
            format_template = '%(levelname)-8s %(name)-30s | %(message)s'

            formatter = logging.Formatter(format_template)
            logging.basicConfig(filename=log_file,
                                level=logging.INFO,
                                format=format_template,
            )
            if debug:
                log.setLevel(logging.DEBUG)

            if log_to_stdout:
                handler = logging.StreamHandler()
                handler.setFormatter(StdoutFormatter())
                log_root = logging.getLogger()
                log_root.addHandler(handler)

        self.processes = {}
        self.address_spaces = []
        #self.default_filter = None

        self.launcher = None
        self.tracer = None
        self.executor = None

        super().__init__(arch,
                         output_directory=DEFAULT_OUTPUT_DIR,
                         configure_logging=False,
        )

        self.message_handlers.update({
            #NewProcessMessage:          self._handle_new_process_message,
            #SyscallEntryMessage:        self._handle_syscall_entry_message,
            #SyscallExitMessage:         self._handle_syscall_exit_message,
            #SyscallForwardEntryMessage: self._handle_syscall_forward_entry_message,
            #SyscallReturnExitMessage:   self._handle_syscall_return_exit_message,
            #SyscallMemoryReadMessage:   self._handle_syscall_memory_read_message,
            #SyscallMemoryWriteMessage:  self._handle_syscall_memory_write_message,
        })

        self.watchmen.add_watch_types([
            #'NewProcess',
            #'SyscallEntry',
            #'SyscallExit',
            #'SyscallForwardEntry',
            #'SyscallReturnExit',
            #'SyscallMemoryRead',
            #'SyscallMemoryWrite',
        ])

        log.info("")
        log.info(f'Chestburster initialized. Output directory is {DEFAULT_OUTPUT_DIR}')
        log.info(f'{"-"*30}')


    def shutdown(self):
        log.info('Shutting down ...')
        super().shutdown()
        if self.launcher is not None:
            self.launcher.stop()
            self.launcher = None


    def init_ptracer(self, executable_path, ip, port):
        """ Initialize the tracer
        """
        self.tracer = TracerTarget(avatar=self, 
                                   ip=ip,
                                   port=port,
                                   executable_path=executable_path,
        )
        self.targets[self.tracer.name] = self.tracer
        self.tracer.init()
        log.info("Tracer target initialized")
        return self.tracer

    def init_executor(self, executable_path, ip, port):
        """ Initialize the executor
        """
        # TODO: add path location where to start the executor?
        self.executor = ExecutorTarget(avatar=self,
                                       ip=ip,
                                       port=port,
                                       executable_path=executable_path,
        )
        self.targets[self.executor.name] = self.executor
        self.executor.init()
        log.info("Executor target initialized")
        return self.executor
