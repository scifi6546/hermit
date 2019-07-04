from pyramid.view import view_config
from pyramid.view import view_defaults
from app import VideoArr
from app import vidArr

from pyramid.response import Response
from pyramid.response import FileResponse
@view_config(route_name='index')
def index(request):
    print("vidArr: ")
    print(vidArr)
    html_str="""
    <video controls class="video">
        <source src="videos/%%URL%%">
    </video>
    """
    str_out="""
    <head>
        <link href="/static/index.css" rel="stylesheet">
    </head>
    """
    for i in vidArr.getVideos():
        str_temp = html_str
        str_out+=str_temp.replace("%%URL%%",i.getUrl());
    print(str_out)
    return Response(str_out)
