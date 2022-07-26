import os
import re
import json
from subprocess import Popen, PIPE

ts_exec = os.getcwd()
if ts_exec.endswith("/test"):
    ts_exec = re.sub("/test","",ts_exec)

ts_exec+="/target/debug/tigershark2"


command = ts_exec

myjson ={\
"name":"Ball",\
"location":"my_ball_location",\
"source":"source_that_created_sphere",\
"datapath":"/my/data/path/mysphere"}

# return tuple with (ErrorCode,output)
def db_insert(myjson):

    try:
        process = Popen([command,"-c","create","-a",json.dumps(myjson)], stdout=PIPE)
        (output, err) = process.communicate()
        exit_code = process.wait()
        output = output.decode('utf-8')
        if exit_code == 0:
            return (0,output)
        else:
            return (1,output)

    except:
        return (1,"Python Popen failed")


out = db_insert(myjson)
print(out)
