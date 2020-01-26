import React from "react";
import { Button, Icon, Container, Segment,Image } from "semantic-ui-react";
import _ from "lodash";
import Axios from 'axios';
let state = {
    serverUrl: "",
    videoList:[],
    playlist:{},
};
class PlaylistEdit extends React.Component {
    constructor(props) {
        super(props);
        this.selectVideo=this.selectVideo.bind(this);
        this.sumbitPlaylist=this.sumbitPlaylist.bind(this);
        this.quit=this.quit.bind(this);

        this.state = _.cloneDeep(state);
        this.state.serverUrl = props.serverUrl;
        this.state.videoList=_.cloneDeep(props.videoList);
        this.state.playlist=_.cloneDeep(props.playlist);
        this.state.quitEdit=props.quitEdit;
        for(let i in this.state.playlist.videos){
            for(let j in this.state.videoList){
                if(this.state.playlist.videos[i].name===this.state.videoList[j].name){
                    this.state.videoList[j].color='yellow';
                    break;
                }
            }
        }
        this.setState({});

    }
    quit(){
        this.state.quitEdit();
    }
    sumbitPlaylist(input) {
        console.log(input.target);
        let videos_in_playlist = []
        for (let i = 0; i < this.state.videoList.length; i++) {
            if (this.state.videoList[i].color === "yellow")
                videos_in_playlist.push(this.state.videoList[i].path)
        }
        let data = { name: this.state.playlist.name, videos: videos_in_playlist };
        Axios.post(this.state.serverUrl + "/api/edit_playlist", data)
        this.quit();
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
    render() {

        return (
            <Container>
                <Button onClick={this.sumbitPlaylist}>
                    Submit Playlist
                </Button>
                <Button onClick={this.quit}>
                    Cancel
                </Button>
                {this.state.videoList.map((vid) =>
                    <Segment key={vid.name} inverted={vid.color} color={vid.color}>

                        <Image src={vid.thumbnail_url} />
                        <Container color="yellow">
                            <Button onClick={this.selectVideo} content='Add To Playlist' icon='add' labelPosition='right' color="red" id={vid.name} />
                        </Container>

                    </Segment>
                )}
            </Container>
        )
    }
}
export default PlaylistEdit;