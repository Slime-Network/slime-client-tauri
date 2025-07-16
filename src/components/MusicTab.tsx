import { Button, Grid, Paper, Stack, TextField, Typography } from '@mui/material';
import React from 'react';

import { useSlimeApi } from '../slime-shared/contexts/SlimeApiContext';
import { RelayGetToSidecarRequest, RelayPostToSidecarRequest } from '../slime-shared/types/slime/SlimeRpcTypes';
import { Song } from '../types/types';
import { SongDisplay } from './SongDisplay';

interface MusicTabProps {
	open: boolean;
}

export const MusicTab = (props: MusicTabProps) => {
	const { open } = props;

	const [songs, setSongs] = React.useState<Song[]>([]);

	const [queueRandomAmount, setQueueRandomAmount] = React.useState<number>(0);

	const [newSong, setNewSong] = React.useState<string>('');

	const { relayGetToSidecar, relayPostToSidecar } = useSlimeApi();

	React.useEffect(() => {
		const fetch = async () => {
			const resp = await relayGetToSidecar({
				port: 5275,
				method: 'songs',
			} as RelayGetToSidecarRequest);
			console.log('songs', resp);
			if (resp.status) console.log('error', resp);
			else setSongs(resp);
		};
		if (open) fetch();
	}, [relayGetToSidecar, open]);

	return (
		<Paper elevation={1} sx={{ width: '100%', height: '100%', p: 2 }}>
			<Grid container spacing={2}>
				<Grid size={{ xs: 12 }}>
					<Typography variant="h5">Music</Typography>
				</Grid>

				<Grid size={{ xs: 6 }}>
					<Stack direction={'row'} spacing={2}>
						<TextField
							label="Add Song"
							variant="outlined"
							fullWidth
							value={newSong}
							onChange={(e) => setNewSong(e.target.value)}
						/>
						<Button
							onClick={async () => {
								const resp = await relayPostToSidecar({
									port: 5275,
									method: 'addYoutubeSong',
									data: {
										url: newSong,
									},
								} as RelayPostToSidecarRequest);
								console.log('addSong', resp);
								setNewSong('');
							}}
						>
							Add
						</Button>
					</Stack>
				</Grid>
				<Grid size={{ xs: 6 }}>
					<Stack direction={'row'} spacing={2}>
						<TextField
							label="Queue Random Songs"
							variant="outlined"
							fullWidth
							type="number"
							value={queueRandomAmount}
							onChange={(e) => setQueueRandomAmount(parseInt(e.target.value, 10))}
						/>
						<Button
							onClick={async () => {
								const resp = await relayPostToSidecar({
									port: 5275,
									method: 'queueRandomSongs',
									data: {
										amount: queueRandomAmount,
									},
								} as RelayPostToSidecarRequest);
								console.log('queueRandomSongs', resp);
								setQueueRandomAmount(0);
							}}
						>
							Play Shuffle
						</Button>
					</Stack>
				</Grid>
				{songs.map((song) => (
					<Grid size={{ xs: 8,  sm: 6,  md: 4, lg: 3, xl:2 }}>
						<SongDisplay song={song} />
					</Grid>
				))}
			</Grid>
		</Paper>
	);
};
