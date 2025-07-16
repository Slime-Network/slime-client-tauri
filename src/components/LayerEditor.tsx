
import { ExpandMore } from '@mui/icons-material';
import DeleteIcon from '@mui/icons-material/Delete';
import { Accordion, AccordionDetails, AccordionSummary, Button, IconButton, TextField } from '@mui/material';
import React from 'react';

import { ElementType, StreamLayer } from '../types/types';
import { BasicElementEditor } from './BasicElementEditor';

interface LayerEditorProps {
    layer: StreamLayer;
    onDelete: () => void;
}

export const LayerEditor = (props: LayerEditorProps) => {

    const { layer, onDelete } = props;

    const [layerName, setLayerName] = React.useState(layer.name);
    const [layerElements, setLayerElements] = React.useState(layer.elements);

    React.useEffect(() => {
        console.log("ffff", layer);
        layer.name = layerName;
        layer.elements = layerElements;
    }, [layerName, layerElements, layer]);

    return (
        <Accordion elevation={2}>
            <AccordionSummary expandIcon={<ExpandMore />}>
                <TextField value={layerName} onChange={(event) => {
                    layer.name = (event.target as HTMLInputElement).value;
                    setLayerName(layer.name);
                }} />
                <IconButton onClick={onDelete}><DeleteIcon /></IconButton>
            </AccordionSummary>
            <AccordionDetails>
                <Button variant="contained" color="primary" onClick={() => {
                    layer.elements.push({ name: "Basic Element", type: ElementType.Basic });
                    setLayerElements([...layer.elements]);
                }}>Add Element</Button>
                {layerElements.map((element) => (
                    <BasicElementEditor element={element} onDelete={() => {
                        layer.elements = layer.elements.filter((e) => e !== element);
                    }} />
                ))}

            </AccordionDetails>
        </Accordion>
    );
};
