import AddIcon from '@mui/icons-material/Add';
import { Box, Fab, Paper, Stack, Typography } from '@mui/material';
import React from 'react';

import { useSlimeApi } from '../slime-shared/contexts/SlimeApiContext';

interface SocialsTabProps {
	open: boolean;
}

export const SocialsTab = (props: SocialsTabProps) => {
	const { open } = props;
	const { relayPostToSidecar } = useSlimeApi();

	React.useEffect(() => {
		if (open) {
			// Any initialization logic can go here if needed
		}
	}, [open]);

	return (
		<Paper elevation={1} sx={{ width: '100%', height: '100%', p: 2 }}>
			<Stack direction={'column'} spacing={2} style={{ padding: '8px' }}>
				<Box sx={{ textAlign: 'center', marginTop: '20px' }}>
					<Typography variant="h4">Socials</Typography>
				</Box>
				<Fab
					size="medium"
					color="primary"
					sx={{ float: 'right', right: '30px', position: 'relative' }}
					onClick={async () => {
						await relayPostToSidecar({
							port: 5275,
							method: 'makeSocialPost',
							data: {},
						});
					}}
				>
					<AddIcon />
				</Fab>
			</Stack>
		</Paper>
	);
};
