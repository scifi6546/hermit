import React from "react";
import _ from "lodash";
import { Button, Image, Container, Segment, Form } from "semantic-ui-react";
import Axios from "axios";
const state = {
    url: "",
    playlistList: [],
    videoList: [],
    edit_playlist: [],
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

        this.getVideos();
        this.getPlaylists();
    }
    async getPlaylists() {
        let res = await Axios.get(this.state.url + "/api/get_playlist_all");
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
    sumbitPlaylist(){
        let videos_in_playlist=[]
        for (let i = 0; i < this.state.videoList.length; i++) {

        }
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
                                <input placeholder="playlist name"></input>
                            </Form.Field>
                            <Button type='submit'>Submit</Button>
                        </Form>
                        {this.state.videoList.map((vid) =>
                            <Segment key={vid.name} inverted color={vid.color}>

                                <Image src={vid.thumbnail_url} />
                                <Container color="yellow">
                                    <Button onClick={this.selectVideo} content='Add To Playlist' icon='add' labelPosition='right' color="red" id={vid.name} />
                                </Container>

                            </Segment>
                        )}
                    </Container>
                )}
            </Container>
        )
    }
}
export default PlaylistList;