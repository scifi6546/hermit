import React from "react";
import { Menu } from "semantic-ui-react";
class PlaylistThumbnailList extends React.Component {
    constructor(props) {
        super(props)
        this.state = { ratio: 3.0, currentRatio: 3.0 };
        this.props = props;
        this.ref = React.createRef();
    }
    componentDidMount() {
        console.log("container width");
        console.log(this.ref.current.offsetWidth);
        console.log("container height");
        console.log(this.ref.current.offsetHeight);
        console.log("container current");
        console.log(this.ref.current);
        console.log(this.ref.current.context);
        this.setState({
            currentRatio: this.ref.current.offsetWidth / this.ref.current.offsetHeight,
        })
    }
    render() {
        if (this.state.currentRatio < this.state.ratio) {
            return (
                <div ref={this.ref}>
                    <Menu style={{ "overflowX": "scroll", "height": "100%" }} vertical>
                        {this.props.children}
                    </Menu>
                </div>
            )
        } else {
            return (
                <div ref={this.ref}>
                    <Menu style={{ "overflowX": "scroll", "height": "100%" }} >
                        {this.props.children}
                    </Menu>
                </div>
            )
        }
    }
}
export default PlaylistThumbnailList;