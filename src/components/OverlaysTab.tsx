import AddIcon from '@mui/icons-material/Add';
import { Box, Fab, Paper, Stack } from '@mui/material';
import React from 'react';
import Split from 'react-split';

import { StreamLayer } from '../types/types';
import { LayerEditor } from './LayerEditor';
import { StreamFrameDisplay } from './StreamFrameDisplay';

export const OverlaysTab = () => {
	const [layers, setLayers] = React.useState<StreamLayer[]>([]);
	const [viewLayer] = React.useState<string | null>(new URLSearchParams(document.location.search).get('layer'));

	React.useEffect(() => {
		setLayers([...layers]);
	}, [layers, layers.length]);
	if (viewLayer) {
		return (
			<Box height={'100vh'}>
				<StreamFrameDisplay layers={layers.filter((layer) => layer.name === viewLayer)} />
			</Box>
		);
	}

	return (
		<Split style={{ width: '100%', display: 'flex', flexDirection: 'row', height: 'calc(100vh - 64px)' }}>
			<Paper style={{ width: '100%', height: '100%' }}>
				<Stack direction={'column'} spacing={2} style={{ padding: '8px' }}>
					{layers.map((layer) => (
						<LayerEditor
							layer={layer}
							onDelete={() => {
								setLayers(layers.filter((l) => l !== layer));
							}}
						/>
					))}
				</Stack>
				<Fab
					size="medium"
					color="primary"
					sx={{ float: 'right', right: '30px', position: 'relative' }}
					onClick={() => {
						setLayers([...layers, { name: 'StreamFrame', elements: [] }]);
					}}
				>
					<AddIcon />
				</Fab>
			</Paper>
			<Box className="transparency-grid">
				<StreamFrameDisplay layers={layers} />
			</Box>
		</Split>
	);
};
