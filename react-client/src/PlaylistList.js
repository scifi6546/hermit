import React from "react";
import _ from "lodash";
import { Button, Image, Container, Segment, Form } from "semantic-ui-react";
import Axios from "axios";
import Playlist from "./PlaylistPlay";
const state = {
    url: "",
    playlistList: [],
    permPlaylistList:[],
    videoList: [],
    edit_playlist: [],
    edit_playlist_name:"",
    playing_playlist:[]
}
class PlaylistList extends React.Component {
    constructor(props) {
        super(props)
        this.state = _.cloneDeep(state);
        this.state.url = props.url;

        this.getPlaylists = this.getPlaylists.bind(this);
        this.getVideos = this.getVideos.bind(this);
        this.makePlaylist = this.makePlaylist.bind(this);
        this.selectVideo = this.selectVideo.bind(this);
        this.sumbitPlaylist=this.sumbitPlaylist.bind(this);
        this.edit_playlist_name=this.edit_playlist_name.bind(this);
        this.activatePlaylist=this.activatePlaylist.bind(this);
        this.quitPlaylist=this.quitPlaylist.bind(this);
        this.getVideos();
        
    }
    componentDidMount(){
        this.getPlaylists();
    }
    async getPlaylists() {
        let res = await Axios.get(this.state.url + "/api/get_playlist_all");
        console.log("geting playlists")
        console.log(res.data);
        this.setState({
            playlistList:_.cloneDeep(res.data),
            permPlaylistList:_.cloneDeep(res.data),

        })
    }
    async getVideos() {
        let res = await Axios.get(this.state.url + "/api/videos");
        console.log(res.data.Ok);
        this.setState({
            videoList: res.data.Ok
        })
    }
    makePlaylist() {
        console.log("Created Playlist")
        this.setState({
            edit_playlist: ["foo"]
        })
    }
    edit_playlist_name(event,target){
        this.setState({
            edit_playlist_name:event.target.value,
        })
    }
    selectVideo(event, target) {
        console.log(event)
        let name = target.id;
        for (let i = 0; i < this.state.videoList.length; i++) {
            if (this.state.videoList[i].name === name) {
                if (this.state.videoList[i].color === "yellow") {
                    this.state.videoList[i].color = null;
                } else {
                    this.state.videoList[i].color = "yellow"
                }
                break;
            }
        }
        this.setState({});
    }
    sumbitPlaylist(input){
        console.log(input.target);
        let videos_in_playlist=[]
        for (let i = 0; i < this.state.videoList.length; i++) {
            if (this.state.videoList[i].color === "yellow")
                videos_in_playlist.push(this.state.videoList[i].path)
        }
        let data = {name:this.state.edit_playlist_name,videos:videos_in_playlist};
        Axios.post(this.state.url+"/api/add_playlist",data)
    }
    activatePlaylist(event){
        console.log(event.target);
        for(let i in this.state.playlistList){
            if(this.state.playlistList[i].name==event.target.id){
                this.setState({
                    playing_playlist:[_.cloneDeep(this.state.playlistList[i])],
                    edit_playlist:[],
                    playlistList:[]
                })
            }
        }

    }
    quitPlaylist(){
        console.log("set state")
        this.setState({
            playing_playlist:[],
            edit_playlist:[],
            playlistList:_.cloneDeep(this.state.permPlaylistList),
        })
    }
    render() {

        return (
            <Container>
                <Button onClick={this.makePlaylist}>Create New Playlist</Button>
                {this.state.edit_playlist.map((foo) =>
                    <Container>
                        <Form onSubmit={this.sumbitPlaylist}>
                            <Form.Field>
                                <label>Playlist Name</label>
                                <input placeholder="playlist name" onChange={this.edit_playlist_name}></input>
                            </Form.Field>
                            <Button type='submit'>Submit</Button>
                        </Form>
                        {this.state.videoList.map((vid) =>
                            <Segment key={vid.name} inverted={vid.color} color={vid.color}>

                                <Image src={vid.thumbnail_url} />
                                <Container color="yellow">
                                    <Button onClick={this.selectVideo} content='Add To Playlist' icon='add' labelPosition='right' color="red" id={vid.name} />
                                </Container>

                            </Segment>
                        )}
                        
                    </Container>
                )}
                {this.state.playlistList.map((play)=>
                            <Segment key={play.name} onClick={this.activatePlaylist} id={play.name}>
                                
                                <Image src={play.videos[0].thumbnail_url} id={play.name}/>
                                {play.name}
                            </Segment>
                )}
                {this.state.playing_playlist.map((play)=>
                <Container>
                        <Playlist playlist={play} quit={this.quitPlaylist}/>
                </Container>
                )}
            </Container>
        )
    }
}
export default PlaylistList;