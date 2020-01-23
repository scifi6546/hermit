import React from "react";
import _ from "lodash";
import { Button, Grid, Icon, Image, Menu, Container } from "semantic-ui-react";
class Playlist extends React.Component {
    constructor(props) {
        super(props)
        this.state = { playlist: _.cloneDeep(props.playlist) }
    }
    render() {

        return (
            <Container>
                <Grid>
                    <Grid.Column width={10}>
                        <Button icon onClick={this.state.quitVideo}>
                            <Icon name="close" />
                        </Button>
                        <video controls>
                            <source src={this.state.playlist.videos[0].url} />
                        </video>
                    </Grid.Column>
                    <Grid.Column width={1}>
                        <Menu vertical>
                            {this.state.playlist.videos.map((video) =>
                                <Image src={video.thumbnail_url} />
                            )}
                        </Menu>
                    </Grid.Column>
                </Grid>
            </Container>
        )
    }
}
export default Playlist;