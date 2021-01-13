import * as React from 'react';

type ElementListProps = {
    onChange: () => any,
}

class ElementList extends React.Component<ElementListProps> {
    render() {
        return (
            <div id="elemlistcontainer" className="container" style={{flex: 2}}>
                <select id="elemlist" className="fillparent" multiple onChange={this.props.onChange}>
                    <option value="none">Nothing</option>
                    <option value="hitcircle">Hitcircle</option>
                    <option value="hitcircleoverlay">Hitcircleoverlay</option>
                    <option value="cursor">Cursor</option>
                    <option value="cursortrail">Cursortrail</option>
                </select>
                {}
            </div>
        );
    }
}

export default ElementList;