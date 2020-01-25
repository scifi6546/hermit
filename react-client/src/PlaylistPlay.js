import React from "react";
import _ from "lodash";
import { Button, Grid, Icon, Image, Menu, Container, Header } from "semantic-ui-react";
class Playlist extends React.Component {
    constructor(props) {
        super(props)
        this.state = { playlist: _.cloneDeep(props.playlist),
        playing_video_index:0}
        this.play_next_vid=this.play_next_vid.bind(this);
    }
    componentDidMount(){
       
    }
    play_next_vid(){
        console.log("video ended");
        let temp_index = this.state.playing_video_index;
        temp_index+=1;
        if(temp_index>=this.state.playlist.videos.length){
            temp_index=0;
        }
        this.setState({
            playing_video_index:temp_index,
        })
        console.log("video ended");
        
    }
    render() {

        return (
            <Container>
                <Button icon onClick={this.state.quitVideo}>
                    <Icon name="close" />
                </Button>
                <video controls onEnded={this.play_next_vid}>
                    <source src={this.state.playlist.videos[this.state.playing_video_index].url} />
                </video>

                <Menu style={{ "overflowX": "scroll", "height": "100%" }}>

                    {this.state.playlist.videos.map((video) =>
                        <Menu.Item active={this.state.playlist.videos[this.state.playing_video_index].name===video.name} key={video.name}>
                            <div style={{ "display": "flex", "flexDirection": "column", "padding": ".00cm", "width": "100%",}}>
                                <div>
                                    <Image src={video.thumbnail_url} style={{"height":"1cm"}}/>
                                </div>
                                <div>
                                    {video.name}
                                </div>
                            </div>

                        </Menu.Item>
                    )}


                </Menu>
            </Container>
        )
    }
}
export default Playlist;