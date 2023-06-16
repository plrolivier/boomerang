"""
"""
import logging

from avatar2.targets import Target, TargetStates
from avatar2.watchmen import watch

from ..protocols.control import ControlChannel, Configuration
from .syscall_fwd import SyscallForwardTarget


log = logging.getLogger(__name__)



class ExecutorTarget(SyscallForwardTarget):
    """ The target which remotly execute the syscall.
        This may be considered to be merged within the TracerTarget
    """

    def __init__(self, *args, **kwargs):

        if 'port' not in kwargs: kwargs['port'] = 31001
        super().__init__(*args, **kwargs)


    @watch('TargetInit')
    def init(self):

        cc = ControlChannel(avatar=self.avatar, origin=self,
                            server_address=self.ip, server_port=self.port,
                            configuration=Configuration.Executor,
                            dbg_executable=self.executable_path)

        is_connected = cc.connect()

        if is_connected:
            log.debug("Successfully connected with Executor")
        else:
            log.error("Fail to connect with Executor")
            return

        self.protocols.set_all(cc)
        self.protocols.ctrl_syscall = cc
        self.update_state(TargetStates.INITIALIZED)
