import os
import subprocess
import magic
class Video:
    def __init__(self,url,name,path,thumbnail_path):
        self.url=url
        self.name=name
        self.path=path
        self.thumbnailPath=thumbnail_path
    def getThumb(self):
        return self.thumbnailPath
    def getUrl(self):
        return self.url
    def getName(self):
        return self.name
    def getFilePath(self):
        return self.path
class VideoArr:
    def __init__(self,video_dir,thumbnail_dir):
        self.thumbnailDir=thumbnail_dir
        if video_dir is None:
            self.videoDir=None
            self.videoFiles=[]
            return
        self.videoDir=""
        self.setVideoPath(video_dir)
        self.genThumbnails()
    def setVideoPath(self,video_dir):
        if(os.path.isdir(video_dir)==False):
            return {"message":"file not found"}
        if(video_dir!=self.videoDir):
            self.videoDir=video_dir
            temp_vids = os.listdir(self.videoDir)
            self.videoFiles=[]
            for i in temp_vids:
                path=os.path.join(self.videoDir,i)
                if os.path.isdir(path) ==False:
                    if magic.from_file(path,mime=True)[0:5]=='video':
                        self.videoFiles.append(Video(i,i,path,None))
            self.genThumbnails()
    def genThumbnails(self):
        for i in range(0,len(self.videoFiles)):
            vid = self.videoFiles[i]
            thumb_path=os.path.join(self.thumbnailDir,vid.getName()+".png")
            print(vid.getFilePath())
            print(thumb_path)
            comm=["bash","-c","ffmpegthumbnailer -i " + str(vid.getFilePath())+ 
                    " -o " + str(thumb_path)+ " -s 800"]
            print(comm)
            subprocess.run(comm)
            self.videoFiles[i].thumbnailPath=thumb_path

    def getThumbnailPath(self):
        return self.thumbnailDir
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


