import bcrypt
def hash_password(pw_in):
    pwhash = bcrypt.hashpw(pw_in.encode('utf8'),bcrypt.gensalt())
    return pwhash.decode('utf8')

USER={"username":"root","passwd":hash_password("password").encode('utf8')}
def check_password(username,pw_check):
    if(username!='root'):
        return False
    return bcrypt.checkpw(pw_check.encode('utf8'),USER['passwd'])

def groupfinder(userid,request):
    return "root"
