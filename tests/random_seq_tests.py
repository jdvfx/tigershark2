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

commands = ["create","latest","update","source","approve","delete"]
commands.append("bullshit")


# print in RED colors (for errors)
def prRed(skk): print("\033[91m {}\033[00m" .format(skk))

# return tuple with (ErrorCode,output)
def tigershark2(mycommand,myjson):

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

asset_names = ["sphere","cube","cone","volume","point","torus","plane","grid","teapot","ball","ground","particle","mouse","bird","cat","dog","car","duck","tree","robot","droid","spaceship","banana","apple","orange","berry","peach","pizza","burger","fries","water","fire","Dust","Explosion","Dark Energy","MagicBeam","Lightning Bolts"]

letters = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ"


for i in range(100):

    mycommand = random.choice(commands)

    randomchar = random.choice(letters)
    a =  random.choice(asset_names) + "_" + randomchar
    l = "/location/"+a
    s = "/source/"+a
    d = "/data/path/"+a
    v = random.randint(0,20)


    if random.random()>0.94:
        l = "bullshit"

    myjson = {"name":a, "location":l,"source":s,"datapath":d, "version":v}

    # execute command with assetjson
    out = tigershark2(mycommand,myjson)

    l1 = ">--- "+str(mycommand) +" "+str(myjson)
    if out[0]!=0:
        prRed(l1)
        prRed(out)
        prRed("!---")
    else:
        print(l1)
        print(out)
        print("!---")


