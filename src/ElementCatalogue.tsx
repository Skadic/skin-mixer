import * as React from 'react';
import CatalogueElement from './CatalogueElement';

type ElementCatalogueProps = {
    elements: Map<string, string>
}

class ElementCatalogue extends React.Component<ElementCatalogueProps> {
    render() {

        let items: Array<React.ReactElement> = [];

        this.props.elements.forEach((imageBase64: string, skinName: string) => {
            items.push(<CatalogueElement skinName={skinName} imgBase64={imageBase64}/>)
        });

        return (
            <div id="elemcatalogue" className="container flexcontainer" style={{flex: 4}}>
                {items}
            </div>
        );
    }
}

export default ElementCatalogue;
