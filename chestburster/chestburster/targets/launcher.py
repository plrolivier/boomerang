import logging
import subprocess
import threading

from ..protocols.ssh_mixin import SshMixin


log = logging.getLogger(__name__)



class LauncherTarget(object):
    """ Used to launch process outside the scope of avatar2.
    """

    def __init__(self, command):
        self._command = command
        self._process = None
        self._thread = threading.Thread(target=self.run)

    def run(self, command=None):
        if command is not None:
            self._command = command
        log.debug(f'Launcher command line: {" ".join(self._command)}')
        self._process = subprocess.Popen(self._command, stdin=subprocess.DEVNULL)
        log.info(f'Launcher process started')

    def start(self):
        self._thread.start()

    def stop(self):
        if self._process:
            ret = self._process.kill()
            if ret is not None:
                log.info(ret)



class LauncherSshTarget(SshMixin, LauncherTarget):
    """ Used to launch process outside the scope of avatar2 through SSH.
    """

    def __init__(self, *args, **kwargs):
        super().__init__(*args, **kwargs)
        self._thread = threading.Thread(target=self.run_ssh, args=[self._command])

    def run(self, command=None):
        if command is not None:
            self._command = command
        log.debug(f'SSH launcher command line: {" ".join(self._command)}')
        self.run_ssh(self._command)
        log.info(f'SSH launcher process started')

    def stop(self):
        self.stop_ssh()
