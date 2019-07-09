import os
import magic
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
    def __init__(self,video_dir):
        if video_dir is None:
            self.videoDir=None
            self.videoFiles=[]
            return
        self.videoDir=video_dir
        temp_vids = os.listdir(self.videoDir)
        self.videoFiles=[]
        for i in temp_vids:
            path=os.path.join(self.videoDir,i)
            if(magic.from_file(path,mime=True)[0:5]=='video'):
                self.videoFiles.append(Video(i,i,path))
    def setVideoPath(self,video_dir):
        if(os.path.isdir(video_dir)==False):
            return {"message":"file not found"}
        if(video_dir!=self.videoDir):
            self.videoDir=video_dir
            temp_vids = os.listdir(self.videoDir)
            self.videoFiles=[]
            for i in temp_vids:
                path=os.path.join(self.videoDir,i)
                self.videoFiles.append(Video(i,i,path))
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


