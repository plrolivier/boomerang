""" Improve logging printing
"""
import logging



class StdoutFormatter(logging.Formatter):

    # TODO: improve logging display
    grey = "\x1b[38;20m"
    blue = "\x1b[34;20m"
    green = "\x1b[32;20m"
    yellow = "\x1b[33;20m"
    red = "\x1b[31;20m"
    bold_red = "\x1b[31;1m"
    reset = "\x1b[0m"
    #format = "%(asctime)s - %(name)s - %(levelname)s - %(message)s (%(filename)s:%(lineno)d)"
    lvlname = '%(levelname)-8s '
    format_template = '%(name)-30s | %(message)s'

    FORMATS = {
        #logging.DEBUG: grey + format_template + reset,
        #logging.INFO: grey + format_template + reset,
        logging.DEBUG: blue + lvlname + reset + format_template,
        logging.INFO: green + lvlname + reset + format_template,
        logging.WARNING: yellow + lvlname + format_template + reset,
        logging.ERROR: red + lvlname + format_template + reset,
        logging.CRITICAL: bold_red + lvlname + format_template + reset
    }

    def format(self, record):
        log_fmt = self.FORMATS.get(record.levelno)
        formatter = logging.Formatter(log_fmt)
        return formatter.format(record)