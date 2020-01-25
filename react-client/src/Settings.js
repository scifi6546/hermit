import React from "react";
import {Button,Icon, Container, Form} from "semantic-ui-react";
import _ from "lodash";
import Axios from 'axios';
let state={
    serverUrl:"",
    thumbnailResolution:0,
    thumb_load:null
};
class Settings extends React.Component{
    constructor(props){
        super(props)
        this.state=_.cloneDeep(state);
        this.state.serverUrl = props.serverUrl;
        this.changeResolution=this.changeResolution.bind(this);
        this.getResolution=this.getResolution.bind(this);
        this.submitResolution=this.submitResolution.bind(this);
        this.getResolution();
    }
    async getResolution(){
        let res = await Axios.get(this.state.serverUrl+"/api/thumbnail_resolution");
        this.setState({
            thumbnailResolution:res.data.thumbnail_resolution, 
        })
    }
    changeResolution(event,bar){

        this.setState({
            thumbnailResolution:event.target.value
        });
    }
    async submitResolution(){
        this.setState({
            thumb_load:true,
        })
        Axios.post(this.state.serverUrl+"/api/settings",{action:"set_resolution",args:String(this.state.thumbnailResolution)})
        this.setState({
            thumb_load:null,
        })
    }
    render(){

        return (
            <Container>
                <Form loading={null}>
                    <Form.Group>
                        <Form.Field>
                            <input type="number" placeholder={this.state.thumbnailResolution} onChange={this.changeResolution}></input>
                            <Button onClick={this.submitResolution}>Set Resolution</Button>
                        </Form.Field>
                    </Form.Group>
                </Form>
            </Container>
        )
    }
}
export default Settings;