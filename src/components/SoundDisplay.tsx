import { Paper } from '@mui/material';

import { Sound } from '../types/types';

interface SoundDisplayProps {
	sound: Sound;
}

export const SoundDisplay = (props: SoundDisplayProps) => {
	const { sound } = props;

	return (
		<Paper elevation={1} sx={{ width: 250 }}>
			{sound.name}
			{sound.duration}
			{sound.source_url}
			{sound.approved}
		</Paper>
	);
};
