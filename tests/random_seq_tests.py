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

commands = ["create","latest","update","source"]

# return tuple with (ErrorCode,output)
def db_insert(mycommand,myjson):

    try:
        process = Popen([command,"-c",mycommand,"-a",json.dumps(myjson)], stdout=PIPE)
        (output, err) = process.communicate()
        exit_code = process.wait()
        output = output.decode('utf-8')
        if exit_code == 0:
            return (0,output,err)
        else:
            return (1,output,err)

    except:
        return (1,"Python Popen failed")

asset_names = ["sphere","cube","cone","volume","point","torus","plane","grid","teapot","ball"]

letters = "abcdefgh"


for i in range(100):

    mycommand = random.choice(commands)

    randomchar = random.choice(letters)
    a=  random.choice(asset_names) + "_" + randomchar
    l = "/location/"+a
    s = "/source/"+a
    d = "/data/path/"+a
    v = random.randint(0,10)

    # match mycommand:
    #     case "create":
    #         print(mycommand, "Create")
    #     case "latest":
    #         print(mycommand, "latest")
    #     case "update":
    #         print(mycommand, "update")
    #     case "delete":
    #         print(mycommand, "delete")
    #     case "source":
    #         print(mycommand, "source")



    myjson = {"name":a, "location":l,"source":s,"datapath":d, "version":v}

    out = db_insert(mycommand,myjson)
    if out[0]!=0:
        print()
        print(">--- ",mycommand,myjson)
        print(out)
        print("!---")

