import { Grid, Paper } from '@mui/material';
import React from 'react';

import { useSlimeApi } from '../slime-shared/contexts/SlimeApiContext';
import { RelayGetToSidecarRequest } from '../slime-shared/types/slime/SlimeRpcTypes';
import { Character } from '../types/types';
// import { CharactersDisplay } from './CharacterDisplay';
import { VoiceControls } from './VoiceControls';

interface CharactersTabProps {
	open: boolean;
}

export const CharactersTab = (props: CharactersTabProps) => {
	const { open } = props;
	const [characters, setCharacters] = React.useState<Character[]>([]);

	const { relayGetToSidecar } = useSlimeApi();

	React.useEffect(() => {
		let isMounted = true;
		const fetch = async () => {
			try {
				const resp = await relayGetToSidecar({
					port: 5275,
					method: 'characters',
				} as RelayGetToSidecarRequest);
				if (isMounted) setCharacters(resp);
			} catch (error) {
				if (isMounted) {
					setTimeout(() => fetch(), 1000);
				}
			}
		};
		if (open) fetch();
		return () => {
			isMounted = false;
		};
	}, [relayGetToSidecar, open]);

	return (
		<Paper elevation={1} sx={{ width: '100%', height: '100%', p: 2 }}>
			<Grid container spacing={2}>
				<Grid size={{ xs: 12 }}>
					<VoiceControls characters={characters} />
				</Grid>
				{/* {characters &&
					characters.map((character) => (
						<Grid item xs={12} key={character.username}>
							<CharactersDisplay
								character={character}
								onDelete={async () => {
									await relayPostToSidecar({
										port: 5275,
										method: 'deleteCharacter',
										data: [character.username],
									} as RelayPostToSidecarRequest);
									setCharacters((prev) => prev.filter((c) => c.username !== character.username));
								}}
								onUpdate={async (updatedCharacter: Character) => {
									await relayPostToSidecar({
										port: 5275,
										method: 'updateCharacter',
										data: [updatedCharacter],
									} as RelayPostToSidecarRequest);
									setCharacters((prev) => prev.map((c) => (c.username === updatedCharacter.username ? updatedCharacter : c)));
								}}
							/>
						</Grid>
					))} */}
			</Grid>
		</Paper>
	);
};
