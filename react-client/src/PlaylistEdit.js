import React from "react";
import { Button, Icon, Container, Segment, Image, Grid } from "semantic-ui-react";
import _ from "lodash";
import Axios from 'axios';
let state = {
    serverUrl: "",
    videoList: [],
    playlist: {},
};
class PlaylistEdit extends React.Component {
    constructor(props) {
        super(props);
        this.selectVideo = this.selectVideo.bind(this);
        this.sumbitPlaylist = this.sumbitPlaylist.bind(this);
        this.quit = this.quit.bind(this);
        this.shiftUp=this.shiftUp.bind(this);
        this.shiftDown=this.shiftDown.bind(this);

        this.state = _.cloneDeep(state);
        this.state.serverUrl = props.serverUrl;
        this.state.videoList = _.cloneDeep(props.videoList);
        this.state.playlist = _.cloneDeep(props.playlist);
        this.state.quitEdit = props.quitEdit;
        for (let i in this.state.playlist.videos) {
            for (let j in this.state.videoList) {
                if (this.state.playlist.videos[i].name === this.state.videoList[j].name) {
                    this.state.videoList[j].color = 'yellow';
                    break;
                }
            }
        }
        this.setState({});


    }
    quit() {
        this.state.quitEdit();
    }
    async sumbitPlaylist(input) {
        console.log(input.target);
        let videos_in_playlist = []
        for (let i = 0; i < this.state.videoList.length; i++) {
            if (this.state.videoList[i].color === "yellow")
                videos_in_playlist.push(this.state.videoList[i].path)
        }
        let data = { name: this.state.playlist.name, videos: videos_in_playlist };
        await Axios.post(this.state.serverUrl + "/api/edit_playlist", data)
        this.quit();
    }
    shiftUp(event, target) {
        for (let i in this.state.videoList) {
            if (this.state.videoList[i].name === event.target.id) {
                if (i > 0) {
                    let old = this.state.videoList[i];
                    let new_v = this.state.videoList[i-1];
                    this.state.videoList[i] = _.cloneDeep(new_v);
                    this.state.videoList[i-1] = _.cloneDeep(old);
                }else{
                    
                    let new_v = this.state.videoList[i];
                    let old = this.state.videoList[this.state.videoList.length-1];
                    this.state.videoList[this.state.videoList.length-1] = _.cloneDeep(new_v);
                    this.state.videoList[i] = _.cloneDeep(old);
                }
                break;
            }
        }
        this.setState({});
    }
    shiftDown(event, target) {
        console.log("id");
        console.log(event.target.id);
        console.log("vid before");
        console.log(this.state.videoList);
        for (let i in this.state.videoList) {
            if (this.state.videoList[i].name === event.target.id) {
                console.log("found video i:")
                console.log(i);
                if (i < this.state.videoList.length-1) {
                    let old = _.cloneDeep(this.state.videoList[i]);
                    let new_v = _.cloneDeep(this.state.videoList[i+1]);
                    this.state.videoList[i ] = _.cloneDeep(new_v);
                    this.state.videoList[i+1] = _.cloneDeep(old);
                }else{
                    let old = this.state.videoList[i];
                    let new_v = this.state.videoList[0];
                    this.state.videoList[i ] = _.cloneDeep(new_v);
                    this.state.videoList[0] = _.cloneDeep(old); 
                }
                break;
            }
        }
        console.log("vid after");
        console.log(this.state.videoList);
        this.setState({});
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
                        <Grid>
                            <Grid.Column>
                                <Button icon id={vid.name} onClick={this.shiftUp}>
                                    <Icon name="caret up" id={vid.name}/>
                                </Button>
                                <Button icon id={vid.name} onClick={this.shiftDown}>
                                    <Icon name="caret down" id={vid.name}/>
                                </Button>
                            </Grid.Column>
                            <Grid.Column>
                                <Image src={vid.thumbnail_url} />
                                <Container color="yellow">
                                    <Button onClick={this.selectVideo} content='Add To Playlist' icon='add' labelPosition='right' color="red" id={vid.name} />
                                </Container>
                            </Grid.Column>
                        </Grid>


                    </Segment>
                )}
            </Container>
        )
    }
}
export default PlaylistEdit;