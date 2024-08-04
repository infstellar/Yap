import os

from .__version__ import __version__

yaml_file = \
r'''
---
# log4rs.yaml
# 检查配置文件变动的时间间隔
refresh_rate: 300 seconds
# appender 负责将日志收集到控制台或文件, 可配置多个
appenders:
  stdout:
    kind: console
    encoder:
      # log 信息模式
      pattern: "{d(%Y-%m-%d-%H-%M-%S)}|{l}|{m}\n"
  collect_info:
    kind: file
    path: "yap_log/collect.log"
    encoder:
      # log 信息模式
      pattern: "{d(%Y-%m-%d-%H-%M-%S)}|{l}|{m}\n"
  file:
    kind: file
    path: "yap_log/log.log"
    encoder:
      # log 信息模式
      pattern: "{d(%Y-%m-%d-%H-%M-%S)}|{l}|{m}\n"
# 对全局 log 进行配置
root:
  level: info
  appenders:
  #  - stdout
    - file

loggers:
  # Route log events sent to the "app::requests" logger to the "requests" appender,
  # and *not* the normal appenders installed at the root
  yap:
    level: error
    appenders:
      - collect_info
    # additive: false
'''

def set_path(file):
    folder = os.path.dirname(__file__)
    
    yaml_path = os.path.join(folder, "log4rs.yaml")
    with open(os.path.abspath(yaml_file), 'w', encoding='utf-8') as f:
        f.write(yaml_file)
    return os.path.abspath(folder)
