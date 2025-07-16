import { Add, PlayArrow, QueuePlayNext } from '@mui/icons-material';
import { Card, CardContent, CardMedia, IconButton, Typography } from '@mui/material';

import { useSlimeApi } from '../slime-shared/contexts/SlimeApiContext';
import { RelayPostToSidecarRequest } from '../slime-shared/types/slime/SlimeRpcTypes';
import { Song } from '../types/types';

interface SongDisplayProps {
	song: Song;
}

export const SongDisplay = (props: SongDisplayProps) => {
	const { song } = props;

	const { relayPostToSidecar } = useSlimeApi();

	return (
		<Card elevation={2} sx={{ width: 250 }}>
			<CardMedia
				component="img"
				height="140"
				image={song.thumbnail_url.replace('maxresdefault', 'mqdefault')}
				alt={song.title}
			/>
			<CardContent>
				<Typography gutterBottom variant="h5" component="div">
					{song.title}
				</Typography>
				<Typography variant="body2" color="text.secondary">
					{song.author}
				</Typography>
				<IconButton
					aria-label="play"
					onClick={async () => {
						await relayPostToSidecar({
							port: 5275,
							method: 'playSong',
							data: {
								song,
							},
						} as RelayPostToSidecarRequest);
					}}
				>
					<PlayArrow />
				</IconButton>
				<IconButton
					aria-label="play next"
					onClick={async () => {
						await relayPostToSidecar({
							port: 5275,
							method: 'pushSong',
							data: {
								song,
							},
						} as RelayPostToSidecarRequest);
					}}
				>
					<QueuePlayNext />
				</IconButton>
				<IconButton
					aria-label="Queue"
					onClick={async () => {
						await relayPostToSidecar({
							port: 5275,
							method: 'queueSong',
							data: {
								song,
							},
						} as RelayPostToSidecarRequest);
					}}
				>
					<Add />
				</IconButton>
			</CardContent>
		</Card>
	);
};
