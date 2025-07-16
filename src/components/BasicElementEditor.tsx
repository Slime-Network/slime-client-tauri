
import { ExpandMore } from '@mui/icons-material';
import DeleteIcon from '@mui/icons-material/Delete';
import { Accordion, AccordionDetails, AccordionSummary, Button, IconButton, TextField } from '@mui/material';
import React from 'react';

import { Element } from '../types/types';


interface BasicElementEditorProps {
    element: Element;
    onDelete: () => void;
}

export const BasicElementEditor = (props: BasicElementEditorProps) => {

    const { element, onDelete } = props;

    const [elementName, setElementName] = React.useState(element.name);

    React.useEffect(() => {
        console.log("Element name changed");
        setElementName(element.name);
    }, [element, element.name]);

    return (
        <Accordion elevation={2}>
            <AccordionSummary expandIcon={<ExpandMore />}>
                <TextField value={elementName} onChange={(event) => {
                    element.name = (event.target as HTMLInputElement).value;
                    setElementName(element.name);
                }} />
                <IconButton onClick={onDelete}><DeleteIcon /></IconButton>
            </AccordionSummary>
            <AccordionDetails>

                <Button variant="contained" color="error" onClick={onDelete}>Delete</Button>
            </AccordionDetails>
        </Accordion>
    );
};
