""" This is the protocol used to communicated over the control channel with libsysforward.
"""
import logging
import socket
import subprocess
import time

from enum import Enum
from threading import Thread

from .ssh_mixin import SshMixin


log = logging.getLogger(__name__)



# Maybe not useful...
#
#class ControlChannelListener(Thread):
#
#    def __init__(self, server_address, server_port, origin=None, queue_maxsize=5):
#
#        super().__init__()
#
#        self._orgin = origin
#
#        self.server_address = server_address
#        self.server_port = server_port
#        self._conn = None
#
#        # A queue used to pass request from the listening thread
#        self._queue_maxsize = queue_maxsize
#        self._response_queue = Queue(maxsize=self._queue_maxsize)
#
#        self._close = Event()
#        self._closed = Event()
#        self._close.clear()
#        self._closed.clear()
#
#
#    def stop(self):
#        """ Stop the listening thread.
#        """
#        self._close.set()
#        self._closed.wait()
#        self._conn = None
#
#
#    def send(self, buf):
#        log.debug(f'Sending {len(buf)} bytes\n{buf.raw}')
#        return self._conn.sendall(buf)
#
#    def recv_into(self, buf, length):
#        ret = self._conn.recv_into(buf, length)
#        log.debug(f'Received {ret} bytes\n{bytes(buf)}')
#        return ret
#
#
#    def run(self):
#        """ Main loop of the thread
#        """
#        log.debug(f'Server loop started in {self.name} '
#                  f'on {self.server_address}:{self.server_port}'
#        )
#
#        with socket.socket(socket.AF_INET, socket.SOCK_STREAM) as sk:
#            sk.setsockopt(socket.SOL_SOCKET, socket.SO_REUSEADDR, 1)
#            sk.bind((self.server_address, self.server_port))
#            sk.settimeout(10)
#            sk.listen(1)
#
#            while True:
#                    if self._close.is_set(): break
#                    try:
#                        conn, addr = sk.accept()
#                    except socket.timeout:
#                        continue
#                    
#                    with conn:
#                        log.info(f'Connected with {addr}')
#                        self._conn = conn
#
#                        while True:
#                            if self._close.is_set(): break
#
#                            ret = self.recv_into(header, HEADER_SIZE)
#                            if not ret: break
#
#                            self.dispatch_packet(header)
#
#                        log.info(f'Connection closed on {addr}')
#                        self._conn = None
#
#        log.debug(f'Server socket {self.server_port} closed')
#        self._closed.set()


class Configuration(Enum):
    Tracer = 0
    Executor = 1



class ControlChannel(Thread):
    """
    """

    def __init__(self, avatar, server_address, server_port, configuration=Configuration.Tracer,
                 dbg_executable='ptracer', additional_args=[],
                 origin=None, output_dir='/tmp/cb'):

        super().__init__()

        self._origin = origin
        self.avatar = avatar
        self.queue = avatar.queue if avatar is not None else None
        self.fast_queue = avatar.fast_queue if avatar is not None else None

        self.configuration = configuration

        #self._server = ControlChannelListener(server_address, server_port, origin)
        self.server_address = server_address
        self.server_port = server_port
        self.socket = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
        self._conn = None
        self._client_address = None

        self._cmdline = [dbg_executable]
        self._cmdline += additional_args
        
        self._dbg = None
        self._start_debugger(output_dir)

        Thread.__init__(self)
        self.daemon = True

    def _start_debugger(self, output_dir):
        """ Start the debugger on the local host
        """
        output_path = f"{output_dir}/{self.configuration.name.lower()}"
        with open(f"{output_path}_out.log", 'wb') as out, \
                open(f"{output_path}_err.log", 'wb') as err:
            log.debug(f"Starting Syscall Debugger with command line: {' '.join(self._cmdline)}")
            self._dbg = subprocess.Popen(self._cmdline, stdout=out, stderr=err)


    def __del__(self):
        self.shutdown()


    def shutdown(self):
        if self._conn:
            self._conn.close()
            self._conn = None
        self.socket.close()
        if self._dbg is not None:
            self._dbg.terminate()
            self._dbg = None


    def connect(self):
        """ Connect to the Debugger
        """
        time.sleep(1)

        if self._dbg.poll() is not None:
            raise RuntimeError("Syscall Debugger error!")

        log.debug(f"Connecting to Syscall Debugger on {self.server_address}:{self.server_port}")
        try:
            self.start()
            return True
        except:
            log.exception(f"Error connecting to Syscall Debugger with port {self.server_port}")
            return False


    def run(self):
        """ The main loop of the background thread communicating with the debugger.
        """
        try:
            self.socket.connect((self.server_address, self.server_port))
            log.debug(f'Connected with {self.server_address}')
            self._conn = self.socket

            while True:
                # In the future, if needed
                time.sleep(10)

        except:
            log.exception(f"Control Channel on port {self.server_port} background thread died")
        log.debug(f"Control Channelon on port {self.server_port} background thread exiting")


    def _send_message(self, message):
        message += '\n'
        message = message.encode('utf-8')
        log.debug(f"Send message: {message}")
        if self._conn:
            self._conn.sendall(message)
        else:
            log.error("No target connected")

    def _receive_message(self):

        if self._conn:
            buffer = self._conn.recv(1024)
            log.debug(f"Message receive: {buffer}")

            if len(buffer) == 0: 
                log.error("File socket is closed")
                return None

            return buffer
            
        else:
            log.error("No target connected")
            return None
            


    def switch_configuration(self):
        # TODO
        pass


    def set_breakpoint(self):
        # TODO
        pass

    def step(self):
        # TODO
        pass

    def cont(self):
        # TODO
        pass

    def stop(self):
        # TODO
        pass


    def read_registers(self):
        # TODO
        pass

    def write_registers(self):
        # TODO
        pass


    def read_memory(self):
        # TODO
        pass

    def write_memory(self):
        # TODO
        pass


    def spawn_process(self, program='', args=[]):
        if self.configuration is Configuration.Tracer:
            return self.tracer_spawn_process(program, args)
        else:
            return self.exec_spawn_process(program, args)

    def tracer_spawn_process(self, program='', args=[]):
        # spawn_process /path/to/bin args1 args2r
        cmd = ['spawn_process', program] + args
        cmd = ' '.join(cmd)
        self._send_message(cmd)
        message = self._receive_message()
        reply = int.from_bytes(message, byteorder='big', signed=True)
        return reply

    def exec_spawn_process(self, program='', args=[]):
        # spawn_process /path/to/bin args1 args2r
        cmd = ['spawn_process', program] + args
        cmd = ' '.join(cmd)
        self._send_message(cmd)
        message = self._receive_message()
        reply = int.from_bytes(message, byteorder='big', signed=True)
        return reply


    def kill_process(self, pid=[]):
        if self.configuration is Configuration.Tracer:
            return self.tracer_kill_process(pid)
        else:
            return self.exec_kill_process(pid)

    def tracer_kill_process(self, pid=[]):
        # kill_process pid1 pid2 ...
        cmd = ['kill_process'] + list(map(str, pid))
        cmd = ' '.join(cmd)
        self._send_message(cmd)
        message = self._receive_message()
        reply = str(message)    # wait for ACK string
        return reply

    def exec_kill_process(self, pid=[]):
        # kill_process pid1 pid2 ...
        cmd = ['kill_process'] + list(map(str, pid))
        cmd = ' '.join(cmd)
        self._send_message(cmd)
        message = self._receive_message()
        reply = str(message)    # wait for ACK string
        return reply


    def tracer_start_tracing(self, pid=[]):
        # start_tracing pid1 pid2 ...
        cmd = ['start_tracing'] + list(map(str, pid))
        cmd = ' '.join(cmd)
        self._send_message(cmd)
        message = self._receive_message()
        reply = str(message)    # wait for ACK string
        return reply

    def tracer_stop_tracing(self, pid=[]):
        # stop_tracing pid1 pid2 ...
        cmd = ['stop_tracing'] + list(map(str, pid))
        cmd = ' '.join(cmd)
        self._send_message(cmd)
        message = self._receive_message()
        reply = str(message)    # wait for ACK string
        return reply


''' Not needed yet:

class SshControlChannel(SshMixin, ControlChannel):
    """ Used to launch the debugger remotly via SSH.
    """

    def __init__(self, *args, **kwargs):
        super().__init__(*args, **kwargs)
        #self._thread = threading.Thread(target=self.run_ssh, args=[self._command])


    def _start_debugger(self, output_dir):
        """ Replace the local launcher with a ssh one
        """
        self.exec_ssh(self._cmdline)
'''