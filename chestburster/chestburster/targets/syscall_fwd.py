"""
"""
import logging

from avatar2.targets import Target, TargetStates
from avatar2.watchmen import watch

from ..protocols.control import ControlChannel, Configuration

log = logging.getLogger(__name__)



class SyscallForwardTarget(Target):
    """ The target which trace syscalls.
        This may be considered to be merged within the EmulatorTarget
    """

    def __init__(self, ip, port, executable_path, executable_args=[], *args, **kwargs):

        super().__init__(*args, **kwargs)

        self.executable_path = executable_path
        self.executable_args = executable_args
        self.ip = ip
        self.port = port
        self.args = args


    # TODO: add decorators
    #@watch('TargetInit')
    #@action_valid_decorator_factory(TargetStates.NOT_RUNNING, 'ctrl_syscall')
    def spawn_process(self, program, args):
        return self.protocols.ctrl_syscall.spawn_process(program, args)

    #@watch('TargetInit')
    #@action_valid_decorator_factory(TargetStates.NOT_RUNNING, 'ctrl_syscall')
    def kill_process(self, pid=[]):
        return self.protocols.ctrl_syscall.kill_process(pid)
