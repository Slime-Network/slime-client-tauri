import AddIcon from '@mui/icons-material/Add';
import ArrowDropDownIcon from '@mui/icons-material/ArrowDropDown';
import ArrowDropUpIcon from '@mui/icons-material/ArrowDropUp';
import ComputerIcon from '@mui/icons-material/Computer';
import DeleteIcon from '@mui/icons-material/Delete';
import ExpandMoreIcon from '@mui/icons-material/ExpandMore';
import PersonIcon from '@mui/icons-material/Person';
import {
	Accordion,
	AccordionDetails,
	AccordionSummary,
	Autocomplete,
	Button,
	Chip,
	IconButton,
	Grid,
	Paper,
	Select,
	Stack,
	Switch,
	TextField,
	Typography,
} from '@mui/material';
import React from 'react';

import { useSlimeApi } from '../slime-shared/contexts/SlimeApiContext';
import { RelayPostToSidecarRequest } from '../slime-shared/types/slime/SlimeRpcTypes';
import { Character } from '../types/types';

interface CharactersDisplayProps {
	character: Character;
	onDelete?: () => void;
	onUpdate?: (character: Character) => void;
}

export const CharactersDisplay = (props: CharactersDisplayProps) => {
	const { character } = props;

	const [history, setHistory] = React.useState<string[]>([]);
	const [historyIndex, setHistoryIndex] = React.useState<number>(0);

	const [speakText, setSpeakText] = React.useState<string>(character.speakText || '');

	const { relayPostToSidecar } = useSlimeApi();

	return (
		<Paper elevation={2} sx={{ p: 2 }}>
			<Grid container spacing={2}>
				<Grid size={{ xs: 12 }}>
					<Typography variant="h4">{character.username}</Typography>
				</Grid>
				<Grid size={{ xs: 12 }}>
					<TextField
						label="Speak"
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
									data: { character: character.username, message: character?.speakText, record: false },
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
				</Grid>
				<Grid size={{ xs: 4 }}>
					<TextField
						label="TTS ID"
						variant="outlined"
						sx={{ width: '100%' }}
						value={character.tts_id}
						onChange={(e) => {
							character.tts_id = parseInt(e.target.value, 10);
						}}
					/>
				</Grid>
				<Grid size={{ xs: 8 }}>
					<Select
						label="Type"
						variant="outlined"
						sx={{ width: '100%' }}
						value={character.type}
						onChange={(e) => {
							character.type = e.target.value as string;
						}}
					>
						<option value={'streamer'}>Streamer</option>
						<option value={'moderator'}>Moderator</option>
						<option value={'chatter'}>Chatter</option>
					</Select>
				</Grid>
				<Grid size={{ xs: 12 }}>
					<Autocomplete
						multiple
						options={[]}
						freeSolo
						value={character.nicknames}
						onChange={(_, value) => {
							character.nicknames = value;
						}}
						getOptionLabel={(option) => option}
						renderTags={(tagValue, getTagProps) =>
							tagValue.map((option, index) => {
								const { key, ...tagProps } = getTagProps({ index });
								return <Chip key={key} label={option} {...tagProps} />;
							})
						}
						renderInput={(params) => <TextField {...params} label="Nicknames" />}
					/>
				</Grid>
				<Grid size={{ xs: 12 }}>
					<Accordion>
						<AccordionSummary expandIcon={<ExpandMoreIcon />}>Knowledge</AccordionSummary>
						<AccordionDetails>
							<Stack direction={'column'} spacing={2}>
								{character.knowledge.map((knowledge) => (
									<Paper elevation={2} sx={{ p: 2 }}>
										<Stack direction={'row'} spacing={2} alignItems={'center'}>
											<TextField
												label="Knowledge"
												multiline
												variant="standard"
												sx={{ width: '100%' }}
												value={knowledge}
												onChange={() => {
													character.knowledge?.splice(character.knowledge.findIndex((m) => m === knowledge) ?? -1, 1, knowledge);
													relayPostToSidecar({
														port: 5275,
														method: 'character',
														data: { character },
													} as RelayPostToSidecarRequest);
												}}
											/>
											<Button
												onClick={() => {
													const index = character.knowledge?.findIndex((m) => m === knowledge);
													if (index !== undefined) {
														character.knowledge?.splice(index, 1);
													}
													relayPostToSidecar({
														port: 5275,
														method: 'character',
														data: { character },
													} as RelayPostToSidecarRequest);
												}}
											>
												<DeleteIcon />
											</Button>
										</Stack>
									</Paper>
								))}
								<IconButton
									onClick={() => {
										character.knowledge.push('');
									}}
								>
									<AddIcon />
								</IconButton>
							</Stack>
						</AccordionDetails>
					</Accordion>
				</Grid>
				<Grid size={{ xs: 12 }}>
					AI {character.isAI ? 'Enabled' : 'Disabled'}
					<Switch
						checked={character.isAI}
						onChange={(e) => {
							character.isAI = e.target.checked;
							relayPostToSidecar({
								port: 5275,
								method: 'character',
								data: { character },
							} as RelayPostToSidecarRequest);
						}}
					/>
				</Grid>
				<Grid size={{ xs: 12 }}>
					{character.isAI && (
						<Accordion>
							<AccordionSummary expandIcon={<ExpandMoreIcon />}>Initial Messages</AccordionSummary>
							<AccordionDetails>
								<Stack direction={'column'} spacing={2}>
									{character.init_messages.map((message) => (
										<Paper elevation={2} sx={{ p: 2 }}>
											<Stack direction={'row'} spacing={2} alignItems={'center'}>
												{message.role === 'user' ? <PersonIcon /> : <ComputerIcon />}
												<TextField
													label={message.role === 'user' ? 'User' : character.username}
													multiline
													variant="standard"
													sx={{ width: '100%' }}
													value={message.content}
													onChange={(e) => {
														character.init_messages?.splice(character.init_messages.findIndex((m) => m === message) ?? -1, 1, {
															role: message.role,
															content: e.target.value,
														});
														relayPostToSidecar({
															port: 5275,
															method: 'character',
															data: { character },
														} as RelayPostToSidecarRequest);
													}}
												/>
												<Stack direction={'column'} spacing={0}>
													<Button
														onClick={() => {
															character.init_messages?.splice(character.init_messages.findIndex((m) => m === message) ?? -1, 1, {
																role: 'user',
																content: '',
															});
															relayPostToSidecar({
																port: 5275,
																method: 'character',
																data: { character },
															} as RelayPostToSidecarRequest);
														}}
													>
														<ArrowDropUpIcon />
													</Button>
													<Button
														onClick={() => {
															character.init_messages.splice(character.init_messages.findIndex((m) => m === message) ?? -1, 1, {
																role: 'assistant',
																content: '',
															});
															relayPostToSidecar({
																port: 5275,
																method: 'character',
																data: { character },
															} as RelayPostToSidecarRequest);
														}}
													>
														<ArrowDropDownIcon />
													</Button>
												</Stack>
												<Button
													onClick={() => {
														const index = character.init_messages?.findIndex((m) => m === message);
														if (index !== undefined) {
															character.init_messages?.splice(index, 1);
														}
														relayPostToSidecar({
															port: 5275,
															method: 'character',
															data: { character },
														} as RelayPostToSidecarRequest);
													}}
												>
													<DeleteIcon />
												</Button>
											</Stack>
										</Paper>
									))}
									<IconButton
										onClick={() => {
											character.init_messages.push({
												role: 'user',
												content: '',
											});
										}}
									>
										<AddIcon />
									</IconButton>
								</Stack>
							</AccordionDetails>
						</Accordion>
					)}
				</Grid>
			</Grid>
		</Paper>
	);
};
