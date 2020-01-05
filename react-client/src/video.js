import React from "react";
import {Button,Icon} from "semantic-ui-react";
class Video extends React.Component{
    constructor(props){
        super(props)
        this.state={url:props.url,quitVideo:props.quitVideo}
    }
    render(){

        return (
            <div>
                <Button icon onClick={this.state.quitVideo}>
                    <Icon name="close"/>
                </Button>
                <video controls>
                    <source src={this.state.url}/>
                </video>
            </div>
        )
    }
}
export default Video;
