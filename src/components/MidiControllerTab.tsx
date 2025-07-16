import { Delete } from '@mui/icons-material';
import { Autocomplete, Box, Button, Chip, Grid, IconButton, Paper, Stack, TextField, Typography } from '@mui/material';
import React from 'react';

import { BasicModal } from '../slime-shared/components/BasicModal';
import { useSlimeApi } from '../slime-shared/contexts/SlimeApiContext';
import { RelayGetToSidecarRequest, RelayPostToSidecarRequest } from '../slime-shared/types/slime/SlimeRpcTypes';
import { MidiMapping, Sound } from '../types/types';
import { SoundDisplay } from './SoundDisplay';

interface MidiControllerTabProps {
	open: boolean;
}

export const MidiControllerTab = (props: MidiControllerTabProps) => {
	const { open } = props;
	const [midiMapping, setMidiMapping] = React.useState<MidiMapping[]>();
	const [currentMidi, setCurrentMidi] = React.useState<any>();

	const [activeMapping, setActiveMapping] = React.useState<MidiMapping>();

	const [sounds, setSounds] = React.useState<Sound[]>([]);

	const [newSound, setNewSound] = React.useState<string>('');
	const [newSoundName, setNewSoundName] = React.useState<string>('');
	const [newSoundStartTime, setNewSoundStartTime] = React.useState<number>(0);
	const [newSoundLength, setNewSoundLength] = React.useState<number>(3);

	const { relayGetToSidecar, relayPostToSidecar } = useSlimeApi();

	const [openAddControlModal, setOpenAddControlModal] = React.useState(false);

	React.useEffect(() => {
		const fetch = async () => {
			const resp = await relayGetToSidecar({
				port: 5275,
				method: 'sounds',
			} as RelayGetToSidecarRequest);
			console.log('sounds', resp);
			console.log('soundsee', midiMapping);
			if (resp.status) console.log('error', resp);
			else setSounds(resp);
		};
		if (open) fetch();
	}, [relayGetToSidecar, open, midiMapping]);

	const keys = [];
	let i = 0;
	while (i < 120) {
		keys.push({ key: i++, white: true });
		keys.push({ key: i++, white: false });
		keys.push({ key: i++, white: true });
		keys.push({ key: i++, white: false });
		keys.push({ key: i++, white: true });
		keys.push({ key: i++, white: true });
		keys.push({ key: i++, white: false });
		keys.push({ key: i++, white: true });
		keys.push({ key: i++, white: false });
		keys.push({ key: i++, white: true });
		keys.push({ key: i++, white: false });
		keys.push({ key: i++, white: true });
	}

	React.useEffect(() => {
		const fetch = async () => {
			const resp = await relayGetToSidecar({
				port: 5275,
				method: 'midiMapping',
			} as RelayGetToSidecarRequest);
			console.log('midi', resp);

			setMidiMapping(resp.mappings);
		};
		if (open) fetch();
	}, [relayGetToSidecar, open]);

	React.useEffect(() => {
		const fetchMidi = async () => {
			const resp = await relayGetToSidecar({
				port: 5275,
				method: 'midi',
			} as RelayGetToSidecarRequest);
			setCurrentMidi(resp);
		};

		if (open) {
			fetchMidi();
		}
		const intervalId = setInterval(fetchMidi, 500);
		return () => clearInterval(intervalId);
	}, [relayGetToSidecar, open]);

	const UpdateControl = async (control: MidiMapping) => {
		if (!midiMapping) {
			return;
		}
		const found = midiMapping.find((c) => c.control === control.control);
		if (!found) {
			midiMapping.push(control);
		}
		setMidiMapping([...midiMapping]);
		await relayPostToSidecar({
			port: 5275,
			method: 'midiMapping',
			data: control,
		});
	};

	const UpdateMapping = async (key: MidiMapping) => {
		if (!midiMapping) {
			return;
		}
		const found = midiMapping.find((k) => k.note === key.note);
		if (!found) {
			midiMapping.push(key);
		}
		setMidiMapping([...midiMapping]);
		await relayPostToSidecar({
			port: 5275,
			method: 'midiMapping',
			data: key,
		});
	};

	return (
		<Paper elevation={1} sx={{ width: '100%', height: '100%', p: 2 }}>
			<BasicModal
				open={openAddControlModal}
				setOpen={setOpenAddControlModal}
				title={'Add Control'}
				message={'Manipulate the control and press Add.'}
				confirmText={'Add'}
				cancelText={'Cancel'}
				successAction={async () => {
					setOpenAddControlModal(false);
					const resp = await relayGetToSidecar({
						port: 5275,
						method: 'midi',
					} as RelayGetToSidecarRequest);
					console.log('midi', resp);
					const newControl = {
						note: -1,
						control: resp.control,
						name: 'New Control',
						type: 'Nothing',
						action: 'Control',
						sources: [],
						selection: 'Random',
						inverted: false,
					} as MidiMapping;
					UpdateControl(newControl);
				}}
			></BasicModal>

			<Stack direction={'column'} spacing={2}>
				<Stack direction={'row'} spacing={2} sx={{ overflow: 'auto' }}>
					{midiMapping &&
						midiMapping.map((control) => {
							if (control.control === -1) return null;
							return (
								<Box
									sx={{
										backgroundColor: currentMidi && control.control === currentMidi.control ? '#444' : '222',
										color: 'white',
										width: '280px',
										minWidth: '280px',
										maxWidth: '280px',
										height: '280px',
										borderRadius: '50%',
										border: 'black 2px solid',
										zIndex: 10,
										display: 'flex',
										flexDirection: 'column',
										justifyContent: 'center',
										alignItems: 'center',
									}}
								>
									<Typography variant="h6">Control {control.control}</Typography>
									<Autocomplete
										options={[
											'Sound Volume',
											'Music Volume',
											'Voice Volume',
											'Voice Pitch',
											'Voice Speed',
											'Tick Frequency',
											'Enable AI',
											'Enable AI Hearing',
											'Nothing',
										]}
										value={control.type}
										onChange={(_e, value) => {
											if (!value) return;
											UpdateControl({
												control: control.control,
												name: value,
												note: -1,
												type: value,
												action: 'Control',
												sources: [],
												selection: 'Random',
												inverted: false,
											});
										}}
										sx={{ width: '200px' }}
										renderInput={(params) => <TextField {...params} label="Name" />}
									/>
									<IconButton
										onClick={async () => {
											await relayPostToSidecar({
												port: 5275,
												method: 'midiMappingDelete',
												data: {
													control: control.control,
													name: control.name,
												},
											});
											setMidiMapping((mm) => mm?.filter((m) => m.control !== control.control));
										}}
									>
										<Delete />
									</IconButton>
								</Box>
							);
						})}
					<Button
						onClick={async () => {
							setOpenAddControlModal(true);
						}}
					>
						Add Control
					</Button>
				</Stack>

				<Stack direction={'row'} spacing={0} height={'300px'} sx={{ overflow: 'auto' }}>
					{keys.map((key) => (
						<>
							{key.white ? (
								<Button
									variant="contained"
									sx={{
										backgroundColor:
											currentMidi && key.key === currentMidi.key
												? '#8f8'
												: midiMapping?.find((k) => k.note === key.key)
												? '#fff'
												: '#ddd',
										height: '100%',
										width: '60px',
										minWidth: '60px',
										maxWidth: '60px',
										paddingTop: '100px',
									}}
									onClick={() => {
										const active = midiMapping?.find((k) => k.note === key.key);
										if (active) {
											setActiveMapping(active);
										} else {
											const newKey = {
												note: key.key,
												control: -1,
												name: '',
												type: 'Play',
												action: 'Press',
												sources: [],
												selection: 'Random',
												inverted: false,
											};
											console.log('newKey', newKey);
											UpdateMapping(newKey);
											setActiveMapping(newKey);
										}
									}}
								>
									<Stack direction={'column'} spacing={1}>
										<Typography>{key.key}</Typography>
										{activeMapping && (
											<Box sx={{ rotate: '90deg' }}>
												<Typography sx={{ paddingLeft: '50px' }}>{midiMapping?.find((k) => k.note === key.key)?.type}</Typography>
											</Box>
										)}
									</Stack>
								</Button>
							) : (
								<div style={{ width: '0px' }}>
									<Button
										variant="contained"
										sx={{
											backgroundColor:
												currentMidi && key.key === currentMidi.key
													? '#8f8'
													: midiMapping?.find((k) => k.note === key.key)
													? '#000'
													: '#333',
											color: 'white',
											width: '40px',
											minWidth: '40px',
											maxWidth: '40px',
											height: '50%',
											right: '20px',
											zIndex: 10,
										}}
									>
										{key.key}
									</Button>
								</div>
							)}
							{(key.key + 1) % 12 === 0 && <div style={{ minWidth: '10px' }} />}
						</>
					))}
				</Stack>

				<Paper elevation={2} sx={{ width: '100%', height: '100%', p: 2 }}>
					<Grid container spacing={2}>
						<Grid size={{ xs: 4 }}>
							<TextField
								label="Sound Name"
								variant="outlined"
								fullWidth
								value={newSoundName}
								onChange={(e) => setNewSoundName(e.target.value)}
							/>
						</Grid>
						<Grid size={{ xs: 4 }}>
							<TextField
								label="Youtube URL"
								variant="outlined"
								fullWidth
								value={newSound}
								onChange={(e) => {
									setNewSound(e.target.value);
								}}
							/>
						</Grid>
						<Grid size={{ xs: 1 }}>
							<TextField
								label="Start Time (seconds)"
								variant="outlined"
								fullWidth
								type="number"
								value={newSoundStartTime}
								onChange={(e) => setNewSoundStartTime(Math.max(0, parseInt(e.target.value, 10)))}
							/>
						</Grid>
						<Grid size={{ xs: 1 }}>
							<TextField
								label="Length (seconds)"
								variant="outlined"
								fullWidth
								type="number"
								value={newSoundLength}
								onChange={(e) => setNewSoundLength(Math.max(1, parseInt(e.target.value, 10)))}
							/>
						</Grid>
						<Grid size={{ xs: 2 }}>
							<Button
								onClick={async () => {
									const resp = await relayPostToSidecar({
										port: 5275,
										method: 'addYoutubeSound',
										data: {
											name: newSoundName,
											url: newSound,
											start_time: newSoundStartTime,
											length: newSoundLength,
										},
									} as RelayPostToSidecarRequest);
									console.log('addSound', resp);
									setNewSound('');
								}}
							>
								Add
							</Button>
						</Grid>
					</Grid>
				</Paper>
				<Grid container spacing={2}>
					<Grid size={{ xs: 6 }}>
						<Paper elevation={2} sx={{ width: '100%', height: '100%', p: 2 }}>
							{activeMapping && (
								<Grid container spacing={2}>
									<Grid size={{ xs: 12 }}>
										<Typography variant="h6">Key {activeMapping.note}</Typography>
									</Grid>
									<Grid size={{ xs: 6 }}>
										<Autocomplete
											options={[
												'Play',
												'Recalibrate',
												'Shush',
												'AI Enabled',
												'AI Hearing Enabled',
												'Clear History',
												'Delete Last Message',
												'Nothing',
											]}
											value={activeMapping.type}
											fullWidth
											onChange={(_e, value) => {
												if (!value) return;
												UpdateMapping({ ...activeMapping, type: value });
											}}
											renderInput={(params) => <TextField {...params} label="Type" />}
										/>
									</Grid>
									<Grid size={{ xs: 6 }}>
										<Autocomplete
											options={['Press', 'Release', 'Toggle', 'Hold']}
											value={activeMapping.action}
											fullWidth
											onChange={(_e, value) => {
												if (!value) return;
												UpdateMapping({ ...activeMapping, action: value });
											}}
											renderInput={(params) => <TextField {...params} label="Action" />}
										/>
									</Grid>
									<Grid size={{ xs: 6 }}>
										<Autocomplete
											options={['Random', 'Sequence']}
											value={activeMapping.selection}
											fullWidth
											onChange={(_e, value) => {
												if (!value) return;
												UpdateMapping({ ...activeMapping, selection: value });
											}}
											renderInput={(params) => <TextField {...params} label="Selection" />}
										/>
									</Grid>
									{activeMapping.type === 'Play' && (
										<Grid size={{ xs: 6 }}>
											<Autocomplete
												options={sounds}
												value={activeMapping.sources}
												getOptionLabel={(option) => option.name}
												fullWidth
												multiple
												onChange={(_e, values) => {
													if (!values) return;
													values.forEach((value) => {
														if (!activeMapping.sources.includes(value)) {
															activeMapping.sources.push(value);
														}
													});
													UpdateMapping({ ...activeMapping });
												}}
												renderInput={(params) => <TextField {...params} label="Sources" />}
												renderTags={(value, getTagProps) =>
													value.map((option, index) => <Chip label={option.name} {...getTagProps({ index })} />)
												}
											/>
										</Grid>
									)}
								</Grid>
							)}
						</Paper>
					</Grid>
					<Grid size={{ xs: 6 }}>
						<Paper elevation={2} sx={{ width: '100%', height: '100%', p: 2 }}>
							<Grid container spacing={2}>
								<Grid size={{ xs: 12 }}>
									<Typography variant="h6">Sounds</Typography>
								</Grid>
								{sounds.map((sound) => (
									<Grid size={{ xs: 6 }}>
										<SoundDisplay sound={sound} />
									</Grid>
								))}
							</Grid>
						</Paper>
					</Grid>
				</Grid>
			</Stack>
		</Paper>
	);
};
