from pyramid.view import view_config
from pyramid.view import view_defaults
from app import VideoArr
from app import vidArr

from pyramid.response import Response
from pyramid.response import FileResponse

from pyramid.security import (remember,forget)

from pyramid.httpexceptions import HTTPFound
from users import (check_password)

@view_defaults(renderer='home.jinja2')
class MainView:
    def __init__(self,request):
        self.request=request
        self.logged_in = request.authenticated_userid
    @property
    def counter(self):
        session = self.request.session
        if 'counter' in session:
            session['counter'] +=1
        else:
            session['counter']=1
        print(session['counter'])
        counter=session['counter']
        return counter
    @view_config(route_name='index',renderer='home.jinja2')
    def index(self):
        if self.logged_in is None:
            return HTTPFound(self.request.route_url("login"))

        videoArr_temp=[]
        for i in vidArr.getVideos():
            videoArr_temp.append({"url":i.getUrl()});
        print(self.request.route_url("logout"))
        return {"LOGOUT_URL":self.request.route_url("logout"),"videos":videoArr_temp}
    @view_config(route_name='login',renderer='login.jinja2')
    def login(self):
        request=self.request
        login_url=request.route_url('login')
        referrer=request.url
        if(referrer==login_url):
            referrer='/'
        came_from = request.params.get('came_from',referrer)
        message=''
        username=''
        password=''
        if 'form.submitted' in request.params:
            username = request.params['username']
            password = request.params['password']
            if(check_password(username,password)):
                print("Password Sucessfull")
                headers=remember(request,username)
                return HTTPFound(location=came_from,headers=headers)
        message = 'Failed Login'
        return {"ACTION_URL":request.application_url+"/login"}
    @view_config(route_name='logout')
    def logout(self):
        request=self.request
        headers=forget(request)
        url=request.route_url('index')
        return HTTPFound(location=url,headers=headers)
