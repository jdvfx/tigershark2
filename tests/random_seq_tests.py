import random
import os
import re
import json
from subprocess import Popen, PIPE

ts_exec = os.getcwd()
if ts_exec.endswith("/tests"):
    ts_exec = re.sub("/tests","",ts_exec)

ts_exec+="/target/debug/tigershark2"




command = ts_exec

print(">> ", command)

commands = ["create","latest","update","source"]

myjson ={\
"name":"Pink",\
"location":"my pink location",\
"source":"source_that_created PINK",\
"datapath":"/my/data/path/pink"}


# return tuple with (ErrorCode,output)
def db_insert(mycommand,myjson):

    try:
        process = Popen([command,"-c",mycommand,"-a",json.dumps(myjson)], stdout=PIPE)
        (output, err) = process.communicate()
        exit_code = process.wait()
        output = output.decode('utf-8')
        if exit_code == 0:
            return (0,output)
        else:
            return (1,output)

    except:
        return (1,"Python Popen failed")

asset_names = ["sphere","cube","cone","volume","point","torus","plane","grid","teapot","ball"]

letters = "abcdefgh"


for i in range(10):

    randomchar = random.choice(letters)
    a=  random.choice(asset_names) + "_" + randomchar
    l = "/location/"+a
    s = "/source/"+a
    d = "/data/path/"+a
    myjson = {"name":a, "location":l,"source":s,"datapath":d}
    mycommand=  random.choice(commands)

    out = db_insert(mycommand,myjson)
    print(">--- ",mycommand,myjson)
    print(out)
    print("!---")

"""


out = db_insert(myjson)
print(out)
"""
