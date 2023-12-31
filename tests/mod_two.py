from rusty_logger import Logger, LogConfig, JsonConfig, LogFileConfig


logger = Logger.get_logger(
    name=__file__,
    config=LogConfig(
        json_config=JsonConfig(),
        file_config=LogFileConfig(
            filename="log/test.log",
        ),
        lock_guard=True,
    ),
)


class TestTwo:
    @staticmethod
    def test_logger():
        logger.info("This is a test log")
