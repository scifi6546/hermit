import React from 'react'
import {Segment,Image, Rating, Dropdown, Button, Form} from 'semantic-ui-react'
import Axios from 'axios';
import _ from 'lodash';

const State= {
    starRating:0,
    toUpdate:[],
    rating:"",
    url:"",
    serverUrl:"",
    name:"",
    thumbnailUrl:"",
    playVideoExt:"",
    description:"",
    path:"",
    ratings:[
        {key:"G",text:"G",value:"G"},
        {key:"PG",text:"PG",value:"PG"},
        {key:"PG-13",text:"PG-13",value:"PG-13"},
        {key:"R",text:"R",value:"R"},
        {key:"Not Rated",text:"Not Rated",value:"Not Rated"},
    ]}
    
class VideoThumbnail extends React.Component{
    constructor(props){
        super(props);
        this.state=_.cloneDeep(State);
        console.log("props")
        console.log(props);
        this.state.serverUrl=props.serverUrl;
        this.state.starRating=props.starRating;
        this.state.rating=props.rating;
        this.state.thumbnailUrl=props.thumbnailUrl;
        this.state.playVideoExt=props.playVideo;
        this.state.updateVideo=props.updateVideo;
        this.state.url=props.url;
        this.state.path=props.path;
        console.log("thumbnail this")
        console.log(this)
       
        //this.state.videoData=_.cloneDeep(props.data);
        //this.state.serverUrl=_.cloneDeep(props.serverUrl);

        this.updateStar=this.updateStar.bind(this);
        this.updateRating=this.updateRating.bind(this);
        this.confirmUpdate=this.confirmUpdate.bind(this);
        this.playVideo=this.playVideo.bind(this);
        //this.temp = this.temp.bind(this);

    }

    updateStar(event,rating){
        //console.log(componentDidMount)
        //this.state.tempStarRating=rating;
        
        this.setState({
            starRating:rating.rating,//rating,
            toUpdate:["going to update"],
        })
        console.log("state after")
        console.log(this.state);
    }
    updateRating(event,data_in){
        this.setState({
            rating:data_in.value,
            toUpdate:["going to update"],
        })
    }
    playVideo(){
        this.state.playVideoExt(this.state.url);
    }
    async confirmUpdate(){
        let temp_des=this.state.description;
        let temp_rating=this.state.rating;
        let temp_star = this.state.starRating;
        this.state.updateVideo(this.state.url,this.state.description,this.state.starRating,
            this.state.rating,this.state.path);

        /*
        let post_struct = {
            path:this.state.videoData.path,
            data:{
                star_rating:this.state.videoData.video_data.star_rating,
                rating:this.state.videoData.video_data.star_rating,
                description:this.state.videoData.video_data.description,
            }
        }
        if(this.state.tempRating!==""){
            post_struct.data.rating=this.state.tempRating;
        }
        if(this.state.tempStarRating!==-1){
            post_struct.data.star_rating=this.state.tempStarRating;
        }
        console.log(post_struct);
        let res = await Axios.post(this.state.serverUrl+"/api/edit_video",post_struct);
        console.log(res);*/
    }
    render(){
        
        return (
            <Segment>
                  <Image src={this.state.thumbnailUrl} onClick={this.playVideo} id={this.state.url}/>
                  {this.state.name}
                  <Segment.Group horizontal>
                    <Segment>
                        <Form>
                        <Rating icon='star' defaultRating={this.state.starRating} maxRating={5} onRate={this.updateStar} key="0"/>
                        <Dropdown clearable options={this.state.ratings} onChange={this.updateRating} defaultValue={this.state.rating}/>
                        
                        {this.state.toUpdate.map((temp)=>
                            <Button onClick={this.confirmUpdate} key="1">
                                Confirm
                            </Button>
                        )}
                        </Form>
                        
                    </Segment>
                  </Segment.Group>
            </Segment>
        )
    }
}
export default VideoThumbnail;