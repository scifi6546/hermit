import React from "react";

import {Menu, Button, Icon, Form, Container,Sidebar } from "semantic-ui-react";
import Server from "./Server";
const State={servers:[],popups:[],tmpIP:"",activeServer:[]};
class App extends React.Component{
    constructor(props){
        super(props);
    }
    render(){
        
        return (
            <Container>
                    <Server url={""}/>
            </Container>
        )
    }
}
export default App;