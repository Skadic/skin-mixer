import React from 'react';
import './App.css';
import ElementList from './ElementList';
import ElementCatalogue from './ElementCatalogue';
import {promisified} from 'tauri/api/tauri';


type AppState = {
    shownElements: Map<string, string>,
};
class App extends React.Component<{}, AppState> {

    constructor(props: any) {
        super(props);
        this.state = {
            shownElements: new Map<string, string>(),
        };
    }


    render() {
        return (
            <div id="topcontainer" className="flexcontainer">
                <ElementList onChange={() => this.handleChange()}/>
                <ElementCatalogue elements={this.state.shownElements}/>
                <div></div>
            </div>
        );
    }


    async handleChange() {
        let elemlist = document.getElementById('elemlist') as HTMLSelectElement;
        if (!elemlist.value) return (<span>No Skin Element Selected</span>);

        return await promisified({
            cmd: "chooseSkinElement", 
            elem_type: elemlist.value
        }).then((response: any) => {

            const { images } = response;

            console.log(JSON.stringify(images));
            

            const map = new Map<string, string>(Object.entries(images));
            this.setState({ shownElements: map });
        }).catch(err => {
            alert("Error handling change: " + err)
        });
    }
}

export default App;
