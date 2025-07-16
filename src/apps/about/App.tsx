import { Box } from '@mui/material';
import { useEffect } from 'react';
import './App.css';

export const App = () => {
	useEffect(() => {
		document.title = `About`;
	}, []);


	return (
		<Box height={'100vh'}>
            <Box>
                <Box component="h3" sx={{ mt: 2, mb: 2 }}>
                    Slime Client v0.3.0 Beta
                </Box>

                <Box component="p" sx={{ mb: 2 }}>
                    Slime Network is a free, open source, federated video game marketplace concept.
                </Box>

                <Box component="p" sx={{ mb: 2 }}>
                    To follow its development, or to contribute, see our public repositories on our{' '}
                    <a href="https://github.com/Slime-Network" target="_blank" rel="noopener noreferrer">
                        Github
                    </a>
                </Box>

                <Box component="p" sx={{ mb: 1 }}>
                    Slime Network is built on:
                </Box>
                <Box component="ul" sx={{ textAlign: 'start', pl: 4, mb: 2 }}>
                    <li>
                        <a href="https://v2.tauri.app/" target="_blank" rel="noopener noreferrer">
                            Tauri
                        </a>
                    </li>
                    <li>
                        <a href="https://react.dev/" target="_blank" rel="noopener noreferrer">
                            React
                        </a>{' '}
                        +{' '}
                        <a href="https://www.typescriptlang.org/" target="_blank" rel="noopener noreferrer">
                            TypeScript
                        </a>
                    </li>
                    <li>
                        <a href="https://www.chia.net/" target="_blank" rel="noopener noreferrer">
                            Chia blockchain
                        </a>
                    </li>
                    <li>
                        <a href="https://walletconnect.network/" target="_blank" rel="noopener noreferrer">
                            WalletConnect v2
                        </a>
                    </li>
                    <li>
                        <a href="https://www.libtorrent.org/" target="_blank" rel="noopener noreferrer">
                            libtorrent
                        </a>
                    </li>
                </Box>

                <Box component="p" sx={{ mb: 2 }}>
                    Thank you for participating in the Beta for Slime.
                </Box>

                <Box component="p">
                    Feedback is welcome! Please come say Hi on our{' '}
                    <a href="https://discord.gg/TUpWukz4pJ" target="_blank" rel="noopener noreferrer">
                        Discord
                    </a>
                </Box>
            </Box>
		</Box>
	);
};
