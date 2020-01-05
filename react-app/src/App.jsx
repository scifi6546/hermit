import React from 'react';
import logo from './logo.svg';
import './App.css';
//import logo from "./logo.svg"
import { Menu, Button, Image, Container} from 'semantic-ui-react'
function App() {
  return (
    <div>
      <Menu inverted>
        <Image src={logo} size="mini" />

      </Menu>
      <Container>
        Hermit is A privacy focused video server for Macos, Windows and Linux.
        
      </Container>
    </div>
  );
}

export default App;
