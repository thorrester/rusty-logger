from rusty_logger import Logger, LogLevel, LogConfig, JsonConfig, LogFileConfig

# This logger will log to stdout at INFO level in json format
logger = Logger.get_logger(
    name=__file__,
    config=LogConfig(
        level=LogLevel.INFO,
        json_config=JsonConfig(flatten=True),
        file_config=LogFileConfig(filename="logs/test.log"),
    ),
)
