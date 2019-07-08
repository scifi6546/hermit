
from wsgiref.simple_server import make_server
from pyramid.config import Configurator
from pyramid.view import view_config
from pyramid.view import view_defaults

from pyramid.response import Response
from pyramid.response import FileResponse


from pyramid.session import SignedCookieSessionFactory

from pyramid.authentication import AuthTktAuthenticationPolicy
from pyramid.authorization import ACLAuthorizationPolicy
from views import (MainView)
from videos import (VideoArr)
if __name__=='__main__':
    my_session_factory = SignedCookieSessionFactory('secret')
    with Configurator(session_factory=my_session_factory) as config:
        
        auth_pol=AuthTktAuthenticationPolicy('secret',hashalg='sha512')
        authz_pol = ACLAuthorizationPolicy()
        config.set_authentication_policy(auth_pol)
        config.set_authorization_policy(authz_pol)

        config.include('pyramid_jinja2')
        config.add_route('video','videos/{url}')

        config.add_route('index','/')
        config.add_route('login','login')
        config.add_route("logout","logout")
        config.add_route("config","config")
        config.add_route("setup","setup")
        config.add_route("adduser","addUser")
        config.add_static_view(name='static',path='../../static')

        #config.add_view(index,route_name='index')
        config.scan('views')
        app=config.make_wsgi_app()
    server = make_server('0.0.0.0',8080,app)
    server.serve_forever()
