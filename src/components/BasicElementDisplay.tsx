
import { } from '@mui/icons-material';
import { Box, Typography } from '@mui/material';
import React from 'react';

import { Element } from '../types/types';


interface BasicElementDisplayProps {
    element: Element;
}

export const BasicElementDisplay = (props: BasicElementDisplayProps) => {

    const { element } = props;

    const [elementName, setElementName] = React.useState(element.name);

    React.useEffect(() => {
        console.log("Element name changed");
        setElementName(element.name);
    }, [element, element.name]);

    return (
        <Box sx={{ background: "grey" }}>
            <Typography sx={{ fontSize: "3vw" }}>{elementName}</Typography>
        </Box >
    );
};
