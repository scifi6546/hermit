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
    def __init__(self,config):
        if config is None:
            self.videoDir=None
            self.thumbnailDir="thumbnails"
            self.videoFiles=[]
            self.playlists=[]
            return
        self.videoDir=""
        self.thumbnailDir=config["thumbnails"]
        self.setVideoPath(config["video_path"])
        self.genThumbnails()
        self.playlists=[]
        self.restorePlaylists(config["playlists"])
    def setVideoPath(self,video_dir):
        print("set video path to "+str(video_dir))
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

    def restorePlaylists(self,playlists):
        for playlist in playlists:
            video_names=[]
            for video in playlist["videos"]:
                video_names.append(video["name"])
            self.makePlaylist(video_names,playlist["name"])
    def makePlaylist(self,video_names,playlist_name):
        temp_playlist={"name":playlist_name,"videos":[]}
        temp_videos=[]
        for name in video_names:
                temp_vid = self.getVideoByName(name)
                if(temp_vid!=None):
                    temp_videos.append(temp_vid)
        temp_playlist["videos"]=temp_videos
        self.playlists.append(temp_playlist)
    def getPlaylists(self):
        return self.playlists
    def getPlaylistsWeb(self):
        out=[]
        for play in self.playlists:
            temp_vids=[]
            for vid in play["videos"]:
                temp_vids.append({"url":vid.getUrl(),"name":vid.getName()})
            out.append({"name":play["name"],"videos":temp_vids})
        return out
    def getPlaylistsConfig(self):
        out=[]
        for play in self.playlists:
            temp_vids=[]
            for vid in play["videos"]:
                temp_vids.append({"url":vid.getUrl(),"name":vid.getName()})
            out.append({"name":play["name"],"videos":temp_vids})
        return out
    def getConfig(self):
        return {"video_path":self.videoDir,"thumbnails":self.thumbnailDir,
                "playlists":self.getPlaylistsConfig()}
