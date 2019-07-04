# package
from wsgiref.simple_server import make_server
from pyramid.config import Configurator
from pyramid.view import view_config
from pyramid.view import view_defaults

from pyramid.response import Response
from pyramid.response import FileResponse
import os
VIDEOS_DIR="../../videos/"

class Video:
    def __init__(self,url,name,path):
        self.url=url
        self.name=name
        self.path=path
    def getUrl(self):
        return self.url
    def getName(self):
        return self.name
    def getFilePath(self):
        return self.path
class VideoArr:
    def __init__(self):
            temp_vids = os.listdir(VIDEOS_DIR)
            self.videoFiles=[]
            for i in temp_vids:
                self.videoFiles.append(Video(i,i,VIDEOS_DIR+i))
    def getVideoByName(self, name: str):
        for i in self.videoFiles:
            if(i.getName()==name):
                return i
    def getVideoByURL(self,url: str):
        for i in self.videoFiles:
            if(i.getUrl()==url):
                return i

    def getVideos(self):
        return self.videoFiles




def video(request):
    print("handled video")
    url = request.matchdict['url']
    print(url)
    temp_vid = vidArr.getVideoByURL(url)
    print(temp_vid)
    return FileResponse(temp_vid.getFilePath())



vidArr=VideoArr()
if __name__=='__main__':
    vidArr = VideoArr()
    print(vidArr)
    with Configurator() as config:
        config.include('pyramid_jinja2')
        config.add_route('videos','videos/{url}')

        config.add_route('index','/')
        config.add_static_view(name='static',path='../../static')

        config.add_view(video,route_name='videos')
        #config.add_view(index,route_name='index')
        config.scan('views')
        app=config.make_wsgi_app()
    server = make_server('0.0.0.0',6543,app)
    server.serve_forever()
