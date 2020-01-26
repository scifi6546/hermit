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
        super(props)
        this.state = _.cloneDeep(state);
        this.state.serverUrl = props.serverUrl;
        this.state.videoList=_.cloneDeep(props.videoList);
        this.state.playlist=_.cloneDeep(props.playlist);

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