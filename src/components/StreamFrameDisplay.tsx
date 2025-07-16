
import { Box, Paper, Stack } from '@mui/material';

import { StreamLayer } from '../types/types';
import { LayerDisplay } from './LayerDisplay';

interface StreamFrameDisplayProps {
    layers: StreamLayer[];
}

export const StreamFrameDisplay = (props: StreamFrameDisplayProps) => {

    const { layers } = props;

    return (
        <Stack width={"100%"} direction={'column'} alignItems={"center"} justifyContent={"center"}>
            {layers.map((layer, index) => (
                <Box key={index} sx={{ width: "90%" }}>
                    <Paper sx={{ width: "fit-content" }}>
                        {layer.name}
                    </Paper>
                    <Box key={index} sx={{ borderStyle: "solid", borderColor: "black" }}>
                        <LayerDisplay layer={layer} />
                    </Box>
                </Box>
            ))
            }
        </Stack >
    );
};
