import * as React from 'react';

function CatalogueElement(props: {skinName: string, imgBase64: string}) {
    const imgSrc = "data:image/gif;base64," + props.imgBase64;

    let imgStyle = {
        backgroundImage: "url(" + imgSrc + ")",
        backgroundRepeat: "no-repeat",
        backgroundPosition: "center",
        backgroundSize: "contain",
    };

    return (
        <button className="catalogueElement">
            <div className="catalogueElementImageContainer" style={imgStyle}>
                
            </div>
            <p style={{flex: 1}}>{props.skinName}</p>
        </button>
    );
}

export default CatalogueElement;