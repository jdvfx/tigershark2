import re
import os
from subprocess import Popen, PIPE

# get tigershark executable
target="debug"
# target="release"

# until I figure out a proper fix...
# pwd is different inside/outside of Neovim when using Git
pwd = re.sub("/python","",os.getcwd())
command = pwd+"/target/"+target+"/tigershark2"


json = '{\
"name":"my new asset",\
"location":"my_location",\
"source":"my_source",\
"datapath":"/my/data/path"}'

# // Popen args
c = [command,"-c","create","-a", json] # CREATE


try:
    process = Popen(c, stdout=PIPE)
    (output, err) = process.communicate()
    exit_code = process.wait()
    output = output.decode('utf-8')
    print("exit code:",exit_code)
    if exit_code == 0:
        print("OK:",output)
    else:
        print("ERR:" , output)

except:
    print("ERR: Popen failed")


