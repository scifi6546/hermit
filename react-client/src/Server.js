import React from 'react';
import Video from "./video";
//import Button from 'blueprintjs';
import { Grid, Container, Form, Segment, Label, Message, Header } from 'semantic-ui-react';
import './App.css';
import Axios from 'axios';
import VideoThumbnail from './VideoThumbnail';
import _ from "lodash";
//const URL="";
let State = {
  serverUrl: "",
  memberItem: <div></div>,
  videos: [],
  videosShown: [],
  loggedIn: ["does not exactly matter"],
  playingVideo: [],
  notSetup: [],
  setup: {
    username: "",
    password: "",
    video_dir: "",
    thumb_res: ""
  },
  setupError: []
}
class Server extends React.Component {

  constructor(props) {

    console.log(props);
    super(props);

    this.changeUsername = this.changeUsername.bind(this);
    this.changePassword = this.changePassword.bind(this);
    this.getVideos = this.getVideos.bind(this);
    this.login = this.login.bind(this);
    this.playVideo = this.playVideo.bind(this);
    this.quitVideo = this.quitVideo.bind(this);
    this.state = _.cloneDeep(State);
    this.state.serverUrl = props.url;
    this.updateVideo = this.updateVideo.bind(this);
    this.changeSetup = this.changeSetup.bind(this);
    this.setup = this.setup.bind(this);
    console.log("state");
    console.log(this.state);
    //do setup stuff
    //this.getLoggedIn();
  }
  componentDidMount() {
    this.getLoggedIn();
    this.getSetup();
  }
  async getSetup() {
    let res = await Axios.get(this.state.serverUrl + "/api/is_setup");
    if (res.data.is_setup === "false") {
      console.log("not setup")
      this.setState({
        loggedIn: [],
        notSetup: ["not"]

      })
    } else {
      this.setState({
        notSetup: []

      })
    }
  }
  async getLoggedIn() {
    let bar = await Axios.get(this.state.serverUrl + "/api/logged_in");
    console.log(bar.data.logged_in);
    if (this.state.notSetup.length > 0) {
      return;
    }
    if (bar.data.logged_in == "true") {
      this.setState({
        loggedIn: [],
      })
      //just for testing
      this.getVideos();
    } else {
      this.setState({
        loggedIn: ["does not exactly matter"],
      })
    }
  }
  async login() {
    console.log(this.state.username);
    console.log(this.state.password);
    let data = { username: this.state.username, password: this.state.password };
    console.log(data);
    let resp = await Axios.post(this.state.serverUrl + "/api/login", data);
    console.log(resp);
    if (resp.data === "logged in sucessfully") {
      this.setState({
        loggedIn: []
      })
      this.getVideos();
    }
  }
  async getVideos() {
    let resp = await Axios.get(this.state.serverUrl + "/api/videos");
    //populate videos
    const temp_data = resp.data.Ok;
    let temp_vid_arr = []
    console.log(temp_data);
    for (let i in temp_data) {
      let vid = temp_data[i];
      console.log("state")
      console.log(this.state);
      console.log("vid")
      console.log(vid);
      vid.url = this.state.serverUrl + vid.url;
      vid.thumbnail_url = this.state.serverUrl + vid.thumbnail_url;
      temp_vid_arr.push(vid);
    }

    this.setState({
      videos: temp_vid_arr,
      videosShown: temp_vid_arr,
    })
    console.log(resp);
  }
  playVideo(url) {
    console.log("playing video: ");
    console.log(url);
    this.setState({
      videosShown: [],
      playingVideo: [this.state.serverUrl + url]
    })
  }
  changeUsername(event) {
    console.log(event.target.value);
    this.setState({
      username: event.target.value,
    })
  }
  changePassword(event) {
    this.setState({
      password: event.target.value,
    })
  }
  changeSetup(event) {
    this.state.setup[event.target.id] = event.target.value;
    this.setState({
    })
  }
  quitVideo(url) {
    this.setState({
      playingVideo: [],
      videosShown: this.state.videos,
    });
  }
  async setup() {
    this.state.setup.thumb_res = Number(this.state.setup.thumb_res);
    let res = await Axios.post(this.state.serverUrl + "/api/setup", this.state.setup)
    if (res.data === "success") {
      this.setState({
        notSetup: []
      })
    } else {
      this.setState({
        setupError: [res.data]
      })
    }
    console.log(res);
  }
  async updateVideo(url, description, starRating, rating, path) {
    let post_struct = {
      path: path,
      data: {
        star_rating: starRating,
        rating: rating,
        description: description,
      }
    }
    console.log(post_struct);
    let res = await Axios.post(this.state.serverUrl + "/api/edit_video", post_struct);
    console.log(res);
  }
  render() {
    const videos = this.state.videosShown;

    return (

      <Container>
        {this.state.loggedIn.map((log) =>
          <Grid textAlign='center' style={{ height: '100vh' }} verticalAlign='middle'>
            <Grid.Column style={{ maxWidth: 450 }}>
              <Form key={0} error onSubmit={this.login}>
                <Header>
                  Log In to Account
            </Header>
                <Form.Input
                  type="text"
                  onChange={this.changeUsername}
                  icon="user"
                />
                <Form.Input
                  type="password"
                  icon="lock"
                  onChange={this.changePassword}
                />
                <Form.Input
                  type="submit"
                />
                {this.state.setupError.map((err) =>
                  <Message content={err} />
                )}
              </Form>
            </Grid.Column>
          </Grid>
        )}
        {this.state.notSetup.map((foo) =>
          <Form onSubmit={this.setup}>

            <Form.Field>
              <label>
                Username
              </label>
              <input type="text" onChange={this.changeSetup} id="username" />
            </Form.Field>

            <Form.Field>
              <label>
                Password
              </label>
              <input type="password" onChange={this.changeSetup} id="password" />
            </Form.Field>

            <Form.Field>
              <label>
                Video Directory
              </label>
              <input type="text" onChange={this.changeSetup} id="video_dir" />
            </Form.Field>
            <Form.Field>
              <label>
                Thumbnail Resolution
              </label>
              <input type="text" onChange={this.changeSetup} id="thumb_res" />
            </Form.Field>
            <Form.Input
              type="submit"
            />
          </Form>



        )}
        <Container>
          <Segment.Group>

            {videos.map((vid, index) =>
              <VideoThumbnail
                playVideo={this.playVideo}
                serverUrl={this.state.serverUrl}
                thumbnailUrl={vid.thumbnail_url}
                url={vid.url}
                updateVideo={this.updateVideo}
                path={vid.path}
                starRating={vid.video_data.star_rating}
                description={vid.video_data.description}
                rating={vid.video_data.rating}
              />
            )

            }


            {_.cloneDeep(this.state.playingVideo).map((vid) =>
              <Video url={vid} quitVideo={this.quitVideo} key={"sd"} />
            )}
          </Segment.Group>
        </Container>
      </Container>

    )



  }
}
export default Server;
