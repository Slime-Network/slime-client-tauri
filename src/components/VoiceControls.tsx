import PersonIcon from '@mui/icons-material/Person';
import { Autocomplete, Box, Button, Chip, CircularProgress, Grid, Paper, Slider, Stack, TextField, Typography } from '@mui/material';
import React from 'react';

import { useSlimeApi } from '../slime-shared/contexts/SlimeApiContext';
import { RelayGetToSidecarRequest, RelayPostToSidecarRequest } from '../slime-shared/types/slime/SlimeRpcTypes';
import { Character } from '../types/types';

interface VoiceControlsProps {
	characters: Character[];
}

export const VoiceControls = (props: VoiceControlsProps) => {
	const { characters } = props;

	const [character, setCharacter] = React.useState<Character>(
		characters[0] || {
			username: '',
			tts_id: 0,
			type: 'streamer',
			nicknames: [],
			knowledge: [],
			isAI: false,
			init_messages: [],
		}
	);

	const [voiceRunning, setVoiceRunning] = React.useState<boolean>(false);

	const [history, setHistory] = React.useState<string[]>([]);
	const [historyIndex, setHistoryIndex] = React.useState<number>(0);

	const [hotwords, setHotwords] = React.useState<string[]>([]);

	const [pitch, setPitch] = React.useState<number>(0.5);
	const [speed, setSpeed] = React.useState<number>(1);
	const [volume, setVolume] = React.useState<number>(1);
	const [lastSpoken, setLastSpoken] = React.useState<string>('');
	const [minConfidence, setMinConfidence] = React.useState<number>(0.5);

	const [speakText, setSpeakText] = React.useState<string>('');

	const { relayPostToSidecar, relayGetToSidecar } = useSlimeApi();

	React.useEffect(() => {
		const interval = setInterval(() => {
			relayGetToSidecar({
				port: 5275,
				method: 'listeningParameters',
			} as RelayGetToSidecarRequest).then((resp) => {
				if (resp.status) {
					setVoiceRunning(false);
					console.error('Error fetching listening Parameters:', resp);
				} else {
					console.log('Listening Parameters:', resp);
					setVoiceRunning(true);
					setLastSpoken(resp.last_text_spoken || '');
					setPitch(resp.pitch || 0.5);
					setSpeed(resp.speed || 1);
					setVolume(resp.volume || 1);
					setMinConfidence(resp.min_confidence || 0.5);
					setHotwords(resp.hotwords || []);
				}
			});
		}, 2000);

		return () => clearInterval(interval);
	}, [relayGetToSidecar]);

	return (
		<Paper elevation={2} sx={{ p: 2 }}>
			<Grid container spacing={2}>
				<Grid size={{ xs: 12 }}>
					<Typography variant="h5">Voice Controls</Typography>
					{!voiceRunning && <Box><CircularProgress />Loading Stream Audio...</Box> }
				</Grid>
				<Grid size={{ xs: 12 }}>
					<Stack direction="row" spacing={1} alignItems="center">
						<TextField
							label="Speak"
							disabled={!voiceRunning}
							variant="outlined"
							sx={{ width: '100%' }}
							value={speakText}
							onChange={(e) => {
								setSpeakText(e.target.value);
							}}
							onKeyDown={(e) => {
								if (e.key === 'Enter') {
									setHistory([...history, speakText]);
									relayPostToSidecar({
										port: 5275,
										method: 'speak',
										data: { character: character.username, message: speakText, record: false },
									} as RelayPostToSidecarRequest);
									setSpeakText('');
								}
								if (e.key === 'ArrowDown') {
									if (historyIndex === history.length) return;
									setHistoryIndex(historyIndex + 1);
									setSpeakText(history[historyIndex + 1]);
								}
								if (e.key === 'ArrowUp') {
									if (historyIndex === 0) return;
									setHistoryIndex(historyIndex - 1);
									setSpeakText(history[historyIndex - 1]);
								}
							}}
							onFocus={() => {
								setHistoryIndex(history.length);
							}}
						/>
					</Stack>
				</Grid>
				<Grid size={{ xs: 6 }}>
					<Autocomplete
						options={characters}
						disabled={!voiceRunning}
						getOptionLabel={(option) => option.username}
						value={character}
						onChange={(_event, newValue) => {
							if (newValue) {
								setCharacter(newValue);
								relayPostToSidecar({
									port: 5275,
									method: 'voiceCharacter',
									data: { character: newValue.username },
								} as RelayPostToSidecarRequest);
							}
						}}
						renderInput={(params) => <TextField {...params} label="Select Character" variant="outlined" />}
						renderOption={(propss, option) => (
							<li {...propss}>
								<Stack direction="row" spacing={1} alignItems="center">
									<PersonIcon />
									<Typography>{option.username}</Typography>
								</Stack>
							</li>
						)}
					/>
				</Grid>
				<Grid size={{ xs: 6 }}>
					<Grid container>
						<Grid size={{ xs: 2 }} sx={{ alignContent: 'center', textAlign: 'center' }}>
							<Typography variant="body2">Pitch</Typography>
						</Grid>
						<Grid size={{ xs: 9 }}>
							<Slider
								disabled={!voiceRunning}
								defaultValue={0.5}
								min={0}
								max={1}
								step={0.05}
								value={pitch}
								marks={[
									{ value: 0, label: 'Low' },
									{ value: 0.5, label: 'Normal' },
									{ value: 1, label: 'High' },
								]}
								sx={{
									'& .MuiSlider-thumb': {
										width: '0px',
										height: '0px',
										opacity: 0,
									},
								}}
								onChange={(_event, newValue) => {
									setPitch(newValue as number);
									relayPostToSidecar({
										port: 5275,
										method: 'pitch',
										data: { pitch: newValue },
									} as RelayPostToSidecarRequest);
								}}
							/>
						</Grid>
						<Grid size={{ xs: 1 }} />
					</Grid>
				</Grid>
				<Grid size={{ xs: 6 }}>
					<Autocomplete
						disabled={!voiceRunning}
						multiple
						options={[]}
						freeSolo
						disablePortal
						value={hotwords}
						onChange={(_, value) => {
							setHotwords(value);
							relayPostToSidecar({
								port: 5275,
								method: 'hotwords',
								data: { hotwords: value },
							} as RelayPostToSidecarRequest);
						}}
						getOptionLabel={(option) => option}
						renderTags={(tagValue: String[], getTagProps) =>
							tagValue.map((option, index) => <Chip size="small" label={option} {...getTagProps({ index })} />)
						}
						renderInput={(params) => <TextField {...params} label="Hotwords" />}
					/>
				</Grid>
				<Grid size={{ xs: 6 }}>
					<Grid container>
						<Grid size={{ xs: 2 }} sx={{ alignContent: 'center', textAlign: 'center' }}>
							<Typography variant="body2">Speed</Typography>
						</Grid>
						<Grid size={{ xs: 9 }}>
							<Slider
								disabled={!voiceRunning}
								defaultValue={1}
								min={0}
								max={2}
								step={0.1}
								value={speed}
								marks={[
									{ value: 0, label: 'Slow' },
									{ value: 1, label: 'Normal' },
									{ value: 2, label: 'Fast' },
								]}
								sx={{
									'& .MuiSlider-thumb': {
										width: '0px',
										height: '0px',
										opacity: 0,
									},
								}}
								onChange={(_event, newValue) => {
									setSpeed(newValue as number);
									relayPostToSidecar({
										port: 5275,
										method: 'speed',
										data: { speed: newValue },
									} as RelayPostToSidecarRequest);
								}}
							/>
						</Grid>
						<Grid size={{ xs: 1 }} />
					</Grid>
				</Grid>
				<Grid size={{ xs: 6 }}>
					<Grid container>
						<Grid size={{ xs: 3 }} sx={{ alignContent: 'center', textAlign: 'center' }}>
							<Typography variant="body2">Min Confidence</Typography>
						</Grid>
						<Grid size={{ xs: 6 }}>
							<Slider
								disabled={!voiceRunning}
								defaultValue={0.5}
								min={0}
								max={1}
								step={0.05}
								value={minConfidence}
								marks={[
									{ value: 0, label: 'Low' },
									{ value: 0.5, label: 'Normal' },
									{ value: 1, label: 'High' },
								]}
								sx={{
									'& .MuiSlider-thumb': {
										width: '0px',
										height: '0px',
										opacity: 0,
									},
								}}
								onChange={(_event, newValue) => {
									setMinConfidence(newValue as number);
									relayPostToSidecar({
										port: 5275,
										method: 'minConfidence',
										data: { min_confidence: newValue },
									} as RelayPostToSidecarRequest);
								}}
							/>
						</Grid>
						<Grid size={{ xs: 1 }} />
						<Grid size={{ xs: 2 }}>
							<Button
								disabled={!voiceRunning}
								variant="contained"
								onClick={async () => {
									await relayPostToSidecar({
										port: 5275,
										method: 'shush',
										data: { character: character.username },
									} as RelayPostToSidecarRequest);
									setSpeakText('');
								}}
							>
								Cancel
							</Button>
						</Grid>
					</Grid>
				</Grid>
				<Grid size={{ xs: 6 }}>
					<Grid container>
						<Grid size={{ xs: 2 }} sx={{ alignContent: 'center', textAlign: 'center' }}>
							<Typography variant="body2">Volume</Typography>
						</Grid>
						<Grid size={{ xs: 9 }}>
							<Slider
								disabled={!voiceRunning}
								defaultValue={1}
								min={0}
								max={1}
								step={0.05}
								value={volume}
								marks={[
									{ value: 0, label: 'Mute' },
									{ value: 1, label: 'Normal' },
								]}
								sx={{
									'& .MuiSlider-thumb': {
										width: '0px',
										height: '0px',
										opacity: 0,
									},
								}}
								onChange={(_event, newValue) => {
									setVolume(newValue as number);
									relayPostToSidecar({
										port: 5275,
										method: 'voiceVolume',
										data: { volume: newValue },
									} as RelayPostToSidecarRequest);
								}}
							/>
						</Grid>
						<Grid size={{ xs: 1 }} />
					</Grid>
				</Grid>
				<Grid size={{ xs: 12 }}>
					<TextField
						label="Last Spoken Text"
						disabled
						variant="outlined"
						fullWidth
						value={lastSpoken}
						inputProps={{ style: { fontSize: 'xx-large' } }}
					/>
				</Grid>
			</Grid>
		</Paper>
	);
};
