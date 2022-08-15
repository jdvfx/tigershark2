
import random
import os
import re
import json
from subprocess import Popen, PIPE

ts_exec = os.getcwd()
if ts_exec.endswith("/tests"):
    ts_exec = re.sub("/tests","",ts_exec)


exe = "release"

ts_exec+="/target/"+exe+"/tigershark2"




command = ts_exec

print(">> ", command)

