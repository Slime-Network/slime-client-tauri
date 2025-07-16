import React from "react";
import { CssBaseline } from '@mui/material';
import { green } from '@mui/material/colors';
import {
  Experimental_CssVarsProvider as CssVarsProvider,
  extendTheme,
} from '@mui/material/styles';
import ReactDOM from "react-dom/client";
import { BrowserRouter, Route, Routes } from "react-router";
import { App as SlimeStreamingTools } from "./apps/slime-streaming-tools/App";
import { App as About } from "./apps/about/App";
import "./styles.css";
import { SlimeApiContextProvider } from "./slime-shared/contexts/SlimeApiContext";

const theme = extendTheme({
  colorSchemes: {
    dark: {
      // palette for dark mode
      palette: {
        primary: {
          main: green[500],
        },
        secondary: {
          main: green[500],
        },
      },
    },
  },
});

ReactDOM.createRoot(document.getElementById("root") as HTMLElement).render(
  <React.StrictMode>
    <CssVarsProvider theme={theme}>
      <CssBaseline />
      <SlimeApiContextProvider>
        <BrowserRouter>
          <Routes>
            <Route index element={<SlimeStreamingTools />} />
            <Route path="about" element={<About />} />
          </Routes>
        </BrowserRouter>
      </SlimeApiContextProvider>
    </CssVarsProvider>
  </React.StrictMode>,
);
