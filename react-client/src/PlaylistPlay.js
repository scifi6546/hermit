import React from "react";
import _ from "lodash";
import { Button, Grid, Icon, Image, Menu, Container, Header } from "semantic-ui-react";
class Playlist extends React.Component {
    constructor(props) {
        super(props)
        this.state = { playlist: _.cloneDeep(props.playlist),
        playing_video:""}
    }
    componentDidMount(){
        this.setState({
            playing_video:this.state.playlist.videos[0].name,
        })
    }
    render() {

        return (
            <Container>
                <Button icon onClick={this.state.quitVideo}>
                    <Icon name="close" />
                </Button>
                <video controls>
                    <source src={this.state.playlist.videos[0].url} />
                </video>

                <Menu style={{ "overflowX": "scroll", "height": "100%" }}>

                    {this.state.playlist.videos.map((video) =>
                        <Menu.Item active={this.state.playing_video===video.name}>
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