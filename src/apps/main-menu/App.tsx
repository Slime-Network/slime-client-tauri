import { Grid, ButtonBase, Typography, styled, Box } from "@mui/material";
import "./App.css";
import React from "react";
import { resolveResource } from "@tauri-apps/api/path";
import { convertFileSrc, invoke } from "@tauri-apps/api/core";

function App() {

  async function openApp(appName: string, title: string, url: string) {
    // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
    const filePath = await convertFileSrc(await resolveResource(url));
    console.log(`Opening app: ${appName}, title: ${title}, url: ${filePath}`);
    await invoke("open_app", { appName, title, url: filePath });
    // await invoke("open_app", { appName, title, url });
  }

  const apps = [
    // {
    //   name: "slime-library",
    //   title: "Library",
    //   url: "./apps/slime-library-dapp/index.html",
    //   logo: "./apps/slime-library-dapp/icon.svg",
    // },
    // {
    //   name: "slime-marketplace",
    //   title: "Slime Marketplace",
    //   url: "./apps/slime-marketplace-dapp/index.html",
    //   logo: "./apps/slime-marketplace-dapp/icon.svg",
    // },
    // {
    //   name: "slime-publishing",
    //   title: "Publishing",
    //   url: "./apps/slime-marketplace-publishing-dapp/index.html",
    //   logo: "./apps/slime-marketplace-publishing-dapp/icon.svg",
    // },
    // {
    //   name: "slime-settings",
    //   title: "Settings",
    //   url: "./apps/slime-settings-app/index.html",
    //   logo: "./apps/slime-settings-app/icon.svg",
    // },
    // {
    //   name: "chia-poker",
    //   title: "Chia Poker", 
    //   url: "./apps/chia-poker/index.html", 
    //   logo: "./apps/chia-poker/icon.svg"
    // },
    {
      name: "slime-streaming-tools",
      title: "Streaming Tools",
      url: "../resources/apps/slime-streaming-tools/index.html",
      logo: "../resources/apps/slime-streaming-tools/icon.svg",
    },
    // {
    //   name: "slime-storefront",
    //   title: "Slime Storefront",
    //   url: "https://api.slimenetwork.org/apps/store/index.html",
    //   logo: "./apps/slime-storefront-app/icon.svg",
    // },
    {
      name: "about-slime",
      title: "About",
      url: "../resources/apps/about/index.html",
      logo: "../resources/apps/about/icon.svg",
    },
  ];

  const ImageButton = styled(ButtonBase)(({ theme }) => ({
    position: 'relative',
    height: 200,
    [theme.breakpoints.down('sm')]: {
      width: '100% !important', // Overrides inline-style
      height: 100,
    },
    '&:hover, &.Mui-focusVisible': {
      zIndex: 1,
      '& .MuiImageBackdrop-root': {
        opacity: 0.15,
      },
      '& .MuiImageMarked-root': {
        opacity: 0,
      },
      '& .MuiTypography-root': {
        border: '4px solid currentColor',
        animation: 'border-rgb-2 9s infinite linear'
      },
    },
  }));
  
  const ImageSrc = styled('span')({
    position: 'absolute',
    left: 0,
    right: 0,
    top: 0,
    bottom: 0,
    backgroundSize: 'cover',
    backgroundPosition: 'center 40%',
  });
  
  const Image = styled('span')(({ theme }) => ({
    position: 'absolute',
    left: 0,
    right: 0,
    top: 0,
    bottom: 0,
    display: 'flex',
    alignItems: 'center',
    justifyContent: 'center',
    color: theme.palette.common.white,
  }));
  
  const ImageBackdrop = styled('span')(({ theme }) => ({
    position: 'absolute',
    left: 0,
    right: 0,
    top: 0,
    bottom: 0,
    backgroundColor: theme.palette.common.black,
    opacity: 0.4,
    transition: theme.transitions.create('opacity'),
  }));
  
  const ImageMarked = styled('span')(({ theme }) => ({
    height: 3,
    width: 18,
    backgroundColor: theme.palette.common.white,
    position: 'absolute',
    bottom: -2,
    left: 'calc(50% - 9px)',
    transition: theme.transitions.create('opacity'),
  }));

  const [initialized, setInitialized] = React.useState(false);
  const [collapseImage, setCollapseImage] = React.useState(false);
  const [logoUrl, setLogoUrl] = React.useState("");

  React.useEffect(() => {
    function handleResize() {
      const windowHeight = window.innerHeight;
      if (windowHeight < 600 || window.innerWidth < 200) {
        setCollapseImage(true);
      } else {
        setCollapseImage(false);
      }
    }

    window.addEventListener("resize", handleResize);

    return () => {
      window.removeEventListener("resize", handleResize);
    };
  }, []);

  React.useEffect(() => {
    const init = async () => {
      const logoUrl = await convertFileSrc(
        await resolveResource("../resources/SlimeNetworkSmallAnim.webp")
      );
      setLogoUrl(logoUrl);
    };
    if (initialized) return;
    setInitialized(true);
    init();
  }, [initialized]);

  return (
    <div className="container">
      {!collapseImage && 
        <Box sx={{height:"30vh", width:"100%", alignItems:"center", display:"flex", justifyContent:"center"}}> 
          <img src={logoUrl} style={{maxWidth:"90%", maxHeight:"90%", alignSelf: 'center'}} />
        </Box>
      }
      {/* <h2>Installed Apps</h2> */}

      <Box sx={{overflowY:"auto", overflowX:"hidden", height:`${collapseImage ? "100vh" : "70vh"}`}}> 
        <Grid container spacing={1}>
          {apps.map((app) => (
            <Grid size={{ xs: 12, sm: 6, md: 4, lg: 3 }}>
              <ImageButton
                focusRipple
                key={app.title}
                onClick={() => openApp(app.name, app.title, app.url)}
                style={{
                  width: '100%',
                }}
              >
                <ImageSrc style={{ backgroundImage: `url(${app.logo})` }} />
                <ImageBackdrop className="MuiImageBackdrop-root" />
                <Image>
                  <Typography
                    component="span"
                    variant="subtitle1"
                    color="inherit"
                    sx={{
                      position: 'relative',
                      p: 4,
                      pt: 2,
                      pb: (theme) => `calc(${theme.spacing(1)} + 6px)`,
                    }}
                  >
                    {app.title}
                    <ImageMarked className="MuiImageMarked-root" />
                  </Typography>
                </Image>
              </ImageButton>
            </Grid>
          ))}
        </Grid>
      </Box>
    </div>
  );
}

export default App;
