"""
"""
import logging

from avatar2.targets import Target, TargetStates
from avatar2.watchmen import watch

from ..protocols.control import ControlChannel, Configuration
from .syscall_fwd import SyscallForwardTarget


log = logging.getLogger(__name__)



class TracerTarget(SyscallForwardTarget):
    """ The target which trace syscalls.
        This may be considered to be merged within the EmulatorTarget
    """

    def __init__(self, *args, **kwargs):

        if 'port' not in kwargs: kwargs['port'] = 31000
        super().__init__(*args, **kwargs)


    @watch('TargetInit')
    def init(self):

        cc = ControlChannel(avatar=self.avatar, origin=self,
                            server_address=self.ip, server_port=self.port,
                            configuration=Configuration.Tracer,
                            dbg_executable=self.executable_path)

        is_connected = cc.connect()

        if is_connected:
            log.debug("Successfully connected with Tracer")
        else:
            log.error("Fail to connect with Tracer")
            return

        self.protocols.set_all(cc)
        self.protocols.ctrl_syscall = cc
        self.update_state(TargetStates.INITIALIZED)

    #@watch('TargetInit')
    #@action_valid_decorator_factory(TargetStates.NOT_RUNNING, 'ctrl_syscall')
    def start_tracing(self, pid=[]):
        return self.protocols.ctrl_syscall.tracer_start_tracing(pid)

    #@watch('TargetInit')
    #@action_valid_decorator_factory(TargetStates.NOT_RUNNING, 'ctrl_syscall')
    def stop_tracing(self, pid=[]):
        return self.protocols.ctrl_syscall.tracer_stop_tracing(pid)