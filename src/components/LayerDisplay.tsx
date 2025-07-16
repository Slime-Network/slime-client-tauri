
import { Box } from '@mui/material';
import React from 'react';

import { StreamLayer } from '../types/types';
import { BasicElementDisplay } from './BasicElementDisplay';

interface LayerDisplayProps {
    layer: StreamLayer;
}

export const LayerDisplay = (props: LayerDisplayProps) => {

    const { layer } = props;

    const [layerName,] = React.useState(layer.name);
    const [layerElements, setLayerElements] = React.useState(layer.elements);

    React.useEffect(() => {
        console.log("Layer name changed");
        setLayerElements(layer.elements);
    }, [layer, layer.elements]);

    console.log(`Layer name:`, layerName);

    return (
        <Box sx={{ width: "100%", aspectRatio: 16 / 9 }}>
            {layerElements.map((element, index) => {

                console.log(`Element type: ${element.type}`);
                switch (element.type) {
                    case "Basic":
                        console.log("Basic element found");
                        return <BasicElementDisplay key={index} element={element} />;
                    default:
                        console.log(`Unknown element type: ${element.type}`);
                        return null;
                }
            })}
        </Box>
    );
};
