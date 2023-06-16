import logging

import paramiko



log = logging.getLogger(__name__)



class SshMixin():
    """ A mixin class adding methods to run and stop a Target via SSH.
    """

    def __init__(self, ssh_hostname='localhost', ssh_port=22,
                 ssh_username='root', ssh_password=None,
                 output_directory="/tmp/chestburster", **kwargs):

        """ Warning: password is conserved in clear text here,
            please consider using identity key instead.
        """
        super().__init__(**kwargs)

        self.ssh_hostname = ssh_hostname
        self.ssh_port = ssh_port
        self.ssh_username = ssh_username
        self.ssh_password = ssh_password

        self.output_directory = output_directory

        self.stdin = None
        self.stdout = None
        self.stderr = None
        self.client = paramiko.SSHClient()


    def exec_ssh(self, command):

        self.client.load_system_host_keys()
        log.info(f'SSH connect to {self.ssh_username}@{self.ssh_hostname} on port {self.ssh_port}')
        self.client.connect(self.ssh_hostname,
                            self.ssh_port,
                            self.ssh_username,
                            self.ssh_password,
        )
        if isinstance(command, list):
            command = ' '.join(command)
        log.debug(f'Executing command: {command}')
        with open("%s/%s_out.txt" % (self.output_directory, __class__.__name__), "w") as out, \
                open("%s/%s_err.txt" % (self.output_directory, __class__.__name__), "w") as err:
            self.stdin, self.stdout, self.stderr = self.client.exec_command(command, get_pty=True)

            for line in iter(self.stdout.readline, ""):
                out.write(line)


    def stop_ssh(self):
        self.client.close()
