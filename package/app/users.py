import bcrypt
def hash_password(pw_in):
    pwhash = bcrypt.hashpw(pw_in.encode('utf8'),bcrypt.gensalt())
    return pwhash.decode('utf8')

def groupfinder(userid,request):
    return "root"

class Users:
    def __init__(self,users_start):
        print("made users")
        self.users=users_start
    def checkPassword(self,username,password):
        for user in self.users: 
            if user["username"]==username:
                if bcrypt.checkpw(password.encode('utf8'),user['passwd'].encode('utf8')):
                    return True
        return False
    def addUser(self,username,password):
        for user in self.users: 
            if user["username"]==username:
                print("user already exists")
                return
        self.users.append({"username":username,
            "passwd": hash_password(password)})
    def isPriviliged(self,username):
        for user in self.users:
            if(user["username"]==username):
                return True
        return False
    def getConfig(self):
        return self.users
    #returns
    #[{"username":"foo"}]
    def getUserInfo(self):
        temp_out = []
        for user in self.users:
            temp_out.append({"username":user["username"]})
        return temp_out
