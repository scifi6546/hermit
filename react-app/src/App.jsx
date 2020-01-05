import React from 'react';
import logo from './logo.svg';
import './App.css';
//import logo from "./logo.svg"
import { Menu, Button, Header, Image, Container} from 'semantic-ui-react'
function App() {
  return (
    <div>
      <Menu inverted>
        <Image src={logo} size="mini" />

      </Menu>
      <Container>
        Hermit is A privacy focused video server for Macos, Windows and Linux.
        It is a light weight, and fast server written in rust. This means that 
        deployment is easy, just run the exe. No python or dependicies needed to
        set up. 

        <Header as="h1">
          Setup
        </Header>
          Just download the latest release and run hermit.exe.

      </Container>
    </div>
  );
}

export default App;