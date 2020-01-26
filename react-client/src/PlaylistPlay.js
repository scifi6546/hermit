import React from "react";
import _ from "lodash";
import { Button, Grid, Icon, Image, Menu, Container, Header } from "semantic-ui-react";
class Playlist extends React.Component {
    constructor(props) {
        super(props)
        this.state = { playlist: _.cloneDeep(props.playlist),
        playing_video_index:0,shouldUpdate:false,quit:props.quit}
        this.play_next_vid=this.play_next_vid.bind(this);
        this.play_video=this.play_video.bind(this);
        this.quitPlaylist=this.quitPlaylist.bind(this);
        
    }
    componentDidMount(){
       
    }
    quitPlaylist(){
        console.log("going to quit");
        this.state.quit();
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
    play_video(event){
        console.log(event.target);
        let video_name = event.target.id;
        let video_index=null;
        for(let i in this.state.playlist.videos){
            if(this.state.playlist.videos[i].name==video_name){
                video_index=i;
            }
        }
        if(video_index!=null){
            this.setState({
                playing_video_index:video_index,
            })
        }
        this.forceUpdate();
    }
    render() {
        return (
            <Container>
                <Button icon onClick={this.quitPlaylist}>
                    <Icon name="close" />
                </Button>
                <video controls 
                    onEnded={this.play_next_vid}
                    src={this.state.playlist.videos[this.state.playing_video_index].url}
                />

                <Menu style={{ "overflowX": "scroll", "height": "100%" }}>

                    {this.state.playlist.videos.map((video) =>
                        <Menu.Item active={this.state.playlist.videos[this.state.playing_video_index].name===video.name} key={video.name} id={video.name} onClick={this.play_video}>
                            <div style={{ "display": "flex", "flexDirection": "column", "padding": ".00cm", "width": "100%",}} id={video.name}>
                                <div id={video.name}>
                                    <Image src={video.thumbnail_url} style={{"height":"1cm"}} id={video.name}/>
                                </div>
                                <div id={video.name}>
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