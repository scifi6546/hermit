import json
import os
CONFIG_PATH=os.environ["HERMIT_CONFIG"]
class Config:
    def __init__(self):
        self.config={}
        try:
            with open(CONFIG_PATH,'r') as config_f:
                print(config_f)
                self.config=json.load(config_f)
                print(self.config)
        except:
            print("Config file not found")
    def write(self,config):
        self.config=config
        try:
            with open(CONFIG_PATH,'w') as config_f:
                json.dump(self.config,config_f)
        except:
            print("file write failed")
    def getConfig(self):
        return self.config
