import { Box, Button, Grid, Stack } from '@mui/material';
import React, { useEffect } from 'react';
import './App.css';

import { CharactersTab } from '../../components/CharactersTab';
import { MainTopBar } from '../../components/MainTopBar';
import { MidiControllerTab } from '../../components/MidiControllerTab';
import { MusicTab } from '../../components/MusicTab';
import { OverlaysTab } from '../../components/OverlaysTab';
import { SocialsTab } from '../../components/SocialsTab';

export const App = () => {
	useEffect(() => {
		document.title = `Streaming Tools`;
	}, []);

	const [tab, setTab] = React.useState<string>('Characters');

	return (
		<Box height={'100vh'}>
			<MainTopBar title={tab} />
			<Grid container sx={{ display: 'flex', flexDirection: 'row' }}>
				<Grid size={{ xs: 2 }}>
					<Stack direction={'column'} spacing={2}>
						<Button
							onClick={() => {
								setTab('Characters');
							}}
						>
							Characters
						</Button>
						<Button
							onClick={() => {
								setTab('Midi Controller');
							}}
						>
							Midi Controller
						</Button>
						<Button
							onClick={() => {
								setTab('Socials');
							}}
						>
							Socials
						</Button>
						<Button
							onClick={() => {
								setTab('Music');
							}}
						>
							Music
						</Button>
						<Button
							onClick={() => {
								setTab('Overlays');
							}}
						>
							Overlays
						</Button>
					</Stack>
				</Grid>
				<Grid size={{ xs: 10 }}>
					{tab === 'Characters' ? (
						<CharactersTab open={tab === 'Characters'} />
					) : tab === 'Midi Controller' ? (
						<MidiControllerTab open={tab === 'Midi Controller'} />
					) : tab === 'Music' ? (
						<MusicTab open={tab === 'Music'} />
					) : tab === 'Socials' ? (
						<SocialsTab open={tab === 'Socials'} />
					) : tab === 'Overlays' ? (
						<OverlaysTab />
					) : (
						<div>Default</div>
					)}
				</Grid>
			</Grid>
		</Box>
	);
};
