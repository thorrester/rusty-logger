from typing import Optional
from rusty_logger import logger, LogConfig, JsonConfig, LogFileConfig


logger.config = LogConfig(
    json_config=JsonConfig(),
    file_config=LogFileConfig(
        filename="log/test.log",
    ),
    lock_guard=True,
)
logger.name = __file__


class TestOne:
    @staticmethod
    def test_logger():
        pointer = logger
        assert pointer == logger
        logger.info("This is a test log")
