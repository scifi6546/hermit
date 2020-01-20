import React from "react";
import _ from "lodash";
import {Button,Icon, Container} from "semantic-ui-react";
import Axios from "axios";
const state={
    url:"",
    playlistList:[],
    videoList:[]
}
class PlaylistList extends React.Component{
    constructor(props){
        super(props)
        this.state=_.cloneDeep(state);
        this.state.url=props.url;

        this.getPlaylists=this.getPlaylists.bind(this);
        this.getVideos=this.getVideos.bind(this);

        this.getVideos();
        this.getPlaylists();
    }
    async getPlaylists(){
        let res = await Axios.get(this.state.url+"/api/get_playlist_all");
    }
    async getVideos(){
        let res = await Axios.get(this.state.url+"/api/videos");
        console.log(res);
        this.setState({
            videoList:res.data
        })
    }

    render(){

        return (
            <Container>
                <Button>Create New Playlist</Button>
            </Container>
        )
    }
}
export default PlaylistList;