import React from "react";

class ImageCard extends React.Component {
  constructor(props) {
    super(props);

    this.state = { spans: 0 };
    this.imageRef = React.createRef();
  }

  componentDidMount() {
    // image has finally loaded. img emits 'load' when it's loaded.
    this.imageRef.current.addEventListener("load", this.setSpans);
  }

  setSpans = () => {
    const height = this.imageRef.current.clientHeight;
    const spans = Math.ceil(height / 10);

    // equivalent to {spans : spans}. same name
    this.setState({ spans });
  };

  render() {
    const { description, url } = this.props.image;
    
    console.log(url);
    return (
      <div style={{ gridRowEnd: `span ${this.state.spans}` }}>
        <img ref={this.imageRef} alt={description} src={url} />
      </div>
    );
  }
}

export default ImageCard;
