import os
from pathlib import Path

class Consts:
    HOME=os.path.join(Path.home(), '.cocmd')
    SOURCE_CONFIG_FILE='cocmd.yaml'
    ALIASES_FILE='aliases.txt'
    SCRIPTS_DIR='scripts'
    DEFAULT_TERMINAL='bash'
    CONFIG_FILE='config.yaml'
    SOURCES_FILE='sources.txt'
    TMP_EXEC_FILE_NAME='cocmd-exec.sh'
    CREDENTIALS_FILE='creds.yamal'