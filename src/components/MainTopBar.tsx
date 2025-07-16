import MenuIcon from '@mui/icons-material/Menu';
import AppBar from '@mui/material/AppBar';
import Box from '@mui/material/Box';
import IconButton from '@mui/material/IconButton';
import Menu from '@mui/material/Menu';
import Toolbar from '@mui/material/Toolbar';
import Typography from '@mui/material/Typography';
import { useState } from 'react';

import ThemeSwitcher from './ThemeSwitcher';

interface MainTopBarProps {
	title: string;
}

export const MainTopBar = (props: MainTopBarProps) => {
	const [anchor2El, setAnchor2El] = useState<null | HTMLElement>(null);
	const isMainMenuOpen = Boolean(anchor2El);

	const handleClick2 = (event: React.MouseEvent<HTMLButtonElement>) => {
		setAnchor2El(event.currentTarget);
	};
	const handleClose2 = () => {
		setAnchor2El(null);
	};

	const mainMenuId = 'primary-search-main';
	const renderMainMenu = (
		<Menu
			anchorEl={anchor2El}
			anchorOrigin={{
				vertical: 'top',
				horizontal: 'right',
			}}
			id={mainMenuId}
			keepMounted
			transformOrigin={{
				vertical: 'top',
				horizontal: 'right',
			}}
			open={isMainMenuOpen}
			onClose={handleClose2}
		>
			<ThemeSwitcher />
		</Menu>
	);

	return (
		<Box sx={{ flexGrow: 1 }}>
			<AppBar position="static">
				<Toolbar>
					<IconButton
						size="large"
						edge="start"
						color="inherit"
						aria-label="open drawer"
						aria-haspopup="true"
						sx={{ mr: 2 }}
						onClick={handleClick2}
					>
						<MenuIcon />
					</IconButton>
					<Typography variant="h6" noWrap component="div" sx={{ display: { xs: 'none', sm: 'block' } }}>
						{props.title}
					</Typography>
				</Toolbar>
			</AppBar>
			{renderMainMenu}
		</Box>
	);
};

export default MainTopBar;
